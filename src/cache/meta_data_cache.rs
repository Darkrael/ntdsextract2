use std::collections::{HashMap, HashSet};
use std::ops::Index;

use getset::Getters;
use lazy_static::lazy_static;

use crate::value::FromValue;
use crate::win32_types::{Rdn, Sid};
use crate::{ntds::NtdsAttributeId, EsedbInfo};

use super::{EsedbRowId, RecordId, RecordPointer};

#[derive(Getters)]
#[getset(get = "pub")]
pub struct DataEntryCore {
    record_ptr: RecordPointer,
    parent: RecordId,
    object_category: Option<RecordId>,
    rdn: Rdn,
    sid: Option<Sid>,
}

lazy_static! {
    static ref EMPTY_HASHSET: HashSet<RecordPointer> = HashSet::new();
}

#[derive(Getters)]
pub struct MetaDataCache {
    records: Vec<DataEntryCore>,
    record_rows: HashMap<RecordId, RecordPointer>,
    children_of: HashMap<RecordId, HashSet<RecordPointer>>,

    #[getset(get = "pub")]
    root: RecordPointer,
}

impl TryFrom<&EsedbInfo<'_>> for MetaDataCache {
    type Error = anyhow::Error;
    fn try_from(info: &EsedbInfo<'_>) -> Result<Self, Self::Error> {
        let record_id_column = *info.mapping().index(NtdsAttributeId::DsRecordId).id();
        let parent_column = *info.mapping().index(NtdsAttributeId::DsParentRecordId).id();
        let rdn_column = *info.mapping().index(NtdsAttributeId::AttRdn).id();
        let object_category_column = *info
            .mapping()
            .index(NtdsAttributeId::AttObjectCategory)
            .id();
        let sid_column = *info.mapping().index(NtdsAttributeId::AttObjectSid).id();

        let mut records = Vec::new();
        let mut record_rows = HashMap::new();
        let mut children_of: HashMap<RecordId, HashSet<RecordPointer>> = HashMap::new();
        let mut root = None;
        let count = info.data_table().count_records()?;
        let bar = crate::create_progressbar(
            "Creating cache for record IDs".to_string(),
            count.try_into()?,
        )?;

        for esedb_row_id in 0..count {
            let record = info.data_table().record(esedb_row_id)?;

            if let Some(parent) = RecordId::from_record_opt(&record, parent_column)? {
                if let Some(record_id) = RecordId::from_record_opt(&record, record_id_column)? {
                    if let Some(rdn) = Rdn::from_record_opt(&record, rdn_column)? {
                        let object_category =
                            RecordId::from_record_opt(&record, object_category_column)?;
                        let sid = Sid::from_record_opt(&record, sid_column)?;

                        let record_ptr = RecordPointer::new(record_id, esedb_row_id.into());

                        if parent.inner() != 0 {
                            children_of.entry(parent).or_default().insert(record_ptr);
                        } else if root.is_some() {
                            panic!("object without parent: '{rdn}' at '{record_ptr}");
                        } else {
                            // check if this really is the root entry
                            if rdn.name() == "$ROOT_OBJECT$" {
                                root = Some(record_ptr);
                            } else {
                                log::warn!("object without parent: '{rdn}' at '{record_ptr}");
                            }
                        }

                        records.push(DataEntryCore {
                            record_ptr,
                            parent,
                            //cn,
                            rdn,
                            object_category,
                            sid,
                        });

                        record_rows.insert(
                            record_id,
                            RecordPointer::new(record_id, esedb_row_id.into()),
                        );
                    } else {
                        log::warn!(
                            "ignoring entry in row {esedb_row_id}: attribute {} (RDN) has no value",
                            Into::<&str>::into(NtdsAttributeId::AttRdn)
                        )
                    }
                } else {
                    log::warn!(
                        "ignoring entry in row {esedb_row_id}: attribute {} (RecordID) has no value",
                        Into::<&str>::into(NtdsAttributeId::DsRecordId)
                    )
                }
            } else {
                log::warn!(
                    "ignoring entry in row {esedb_row_id}: attribute {} (ParentRecordId) has no value",
                    Into::<&str>::into(NtdsAttributeId::DsParentRecordId)
                )
            }

            bar.inc(1);
        }
        bar.finish_and_clear();

        Ok(Self {
            records,
            record_rows,
            children_of,
            root: root.expect("no root object found"),
        })
    }
}

impl Index<&EsedbRowId> for MetaDataCache {
    type Output = DataEntryCore;

    fn index(&self, index: &EsedbRowId) -> &Self::Output {
        &self.records[index.inner() as usize]
    }
}

impl Index<&RecordPointer> for MetaDataCache {
    type Output = DataEntryCore;

    fn index(&self, index: &RecordPointer) -> &Self::Output {
        &self[index.esedb_row()]
    }
}

impl MetaDataCache {
    pub fn iter(&self) -> impl Iterator<Item = &DataEntryCore> {
        self.records.iter()
    }

    pub fn children_of(&self, parent: &RecordPointer) -> impl Iterator<Item = &DataEntryCore> {
        self.children_ptr_of(parent)
            .map(|ptr| &self[ptr.esedb_row()])
    }

    pub fn children_ptr_of(&self, parent: &RecordPointer) -> impl Iterator<Item = &RecordPointer> {
        self.children_of
            .get(parent.ds_record_id())
            .unwrap_or(&EMPTY_HASHSET)
            .iter()
    }

    pub fn entries_with_rid(&self, rid: u32) -> impl Iterator<Item = &DataEntryCore> + '_ {
        self.records.iter().filter(move |r| match r.sid() {
            Some(sid) => sid.get_rid() == &rid,
            _ => false,
        })
    }

    pub fn entries_of_type(&self, ot: &RecordId) -> impl Iterator<Item = &DataEntryCore> + '_ {
        let ot = *ot;
        self.records
            .iter()
            .filter(move |r| match r.object_category() {
                Some(oc) => *oc == ot,
                _ => false,
            })
    }

    pub fn entries_of_types(
        &self,
        ot: HashSet<RecordId>,
    ) -> impl Iterator<Item = &DataEntryCore> + '_ {
        self.records
            .iter()
            .filter(move |r| match r.object_category() {
                Some(oc) => ot.contains(oc),
                _ => false,
            })
    }

    pub fn ptr_from_row(&self, row: &EsedbRowId) -> &RecordPointer {
        self[row].record_ptr()
    }

    pub fn ptr_from_id(&self, id: &RecordId) -> Option<&RecordPointer> {
        self.record_rows.get(id)
    }

    pub fn record(&self, index: &RecordId) -> Option<&DataEntryCore> {
        match self.record_rows.get(index) {
            Some(ptr) => self.records.get(ptr.esedb_row().inner() as usize),
            None => None,
        }
    }
}

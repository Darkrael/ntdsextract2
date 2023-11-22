use strum::{EnumString, IntoStaticStr};

#[derive(IntoStaticStr, EnumString, Debug, Eq, PartialEq, Hash, Clone)]
#[strum(use_phf)]
pub enum NtdsAttributeId {

    #[strum(serialize = "ATTq589983")]
    AttAccountExpires = 0x9009f,

    #[strum(serialize = "ATTm591131")]
    AttAccountNameHistory = 0x9051b,

    #[strum(serialize = "ATTq590584")]
    AttAcsAggregateTokenRatePerUser = 0x902f8,

    #[strum(serialize = "ATTq590590")]
    AttAcsAllocableRsvpBandwidth = 0x902fe,

    #[strum(serialize = "ATTj590603")]
    AttAcsCacheTimeout = 0x9030b,

    #[strum(serialize = "ATTj590581")]
    AttAcsDirection = 0x902f5,

    #[strum(serialize = "ATTj590602")]
    AttAcsDsbmDeadtime = 0x9030a,

    #[strum(serialize = "ATTj590600")]
    AttAcsDsbmPriority = 0x90308,

    #[strum(serialize = "ATTj590601")]
    AttAcsDsbmRefresh = 0x90309,

    #[strum(serialize = "ATTi590594")]
    AttAcsEnableAcsService = 0x90302,

    #[strum(serialize = "ATTi590723")]
    AttAcsEnableRsvpAccounting = 0x90383,

    #[strum(serialize = "ATTi590592")]
    AttAcsEnableRsvpMessageLogging = 0x90300,

    #[strum(serialize = "ATTj590593")]
    AttAcsEventLogLevel = 0x90301,

    #[strum(serialize = "ATTm590608")]
    AttAcsIdentityName = 0x90310,

    #[strum(serialize = "ATTq590721")]
    AttAcsMaxAggregatePeakRatePerUser = 0x90381,

    #[strum(serialize = "ATTj590585")]
    AttAcsMaxDurationPerFlow = 0x902f9,

    #[strum(serialize = "ATTj590725")]
    AttAcsMaxNoOfAccountFiles = 0x90385,

    #[strum(serialize = "ATTj590598")]
    AttAcsMaxNoOfLogFiles = 0x90306,

    #[strum(serialize = "ATTq590591")]
    AttAcsMaxPeakBandwidth = 0x902ff,

    #[strum(serialize = "ATTq590583")]
    AttAcsMaxPeakBandwidthPerFlow = 0x902f7,

    #[strum(serialize = "ATTj590726")]
    AttAcsMaxSizeOfRsvpAccountFile = 0x90386,

    #[strum(serialize = "ATTj590599")]
    AttAcsMaxSizeOfRsvpLogFile = 0x90307,

    #[strum(serialize = "ATTq591137")]
    AttAcsMaxTokenBucketPerFlow = 0x90521,

    #[strum(serialize = "ATTq590582")]
    AttAcsMaxTokenRatePerFlow = 0x902f6,

    #[strum(serialize = "ATTq591138")]
    AttAcsMaximumSduSize = 0x90522,

    #[strum(serialize = "ATTq591141")]
    AttAcsMinimumDelayVariation = 0x90525,

    #[strum(serialize = "ATTq591140")]
    AttAcsMinimumLatency = 0x90524,

    #[strum(serialize = "ATTq591139")]
    AttAcsMinimumPolicedSize = 0x90523,

    #[strum(serialize = "ATTq591144")]
    AttAcsNonReservedMaxSduSize = 0x90528,

    #[strum(serialize = "ATTq591145")]
    AttAcsNonReservedMinPolicedSize = 0x90529,

    #[strum(serialize = "ATTq591142")]
    AttAcsNonReservedPeakRate = 0x90526,

    #[strum(serialize = "ATTq591143")]
    AttAcsNonReservedTokenSize = 0x90527,

    #[strum(serialize = "ATTq590604")]
    AttAcsNonReservedTxLimit = 0x9030c,

    #[strum(serialize = "ATTq590722")]
    AttAcsNonReservedTxSize = 0x90382,

    #[strum(serialize = "ATTq590589")]
    AttAcsPermissionBits = 0x902fd,

    #[strum(serialize = "ATTm590596")]
    AttAcsPolicyName = 0x90304,

    #[strum(serialize = "ATTj590588")]
    AttAcsPriority = 0x902fc,

    #[strum(serialize = "ATTm590724")]
    AttAcsRsvpAccountFilesLocation = 0x90384,

    #[strum(serialize = "ATTm590597")]
    AttAcsRsvpLogFilesLocation = 0x90305,

    #[strum(serialize = "ATTj590586")]
    AttAcsServiceType = 0x902fa,

    #[strum(serialize = "ATTm590580")]
    AttAcsTimeOfDay = 0x902f4,

    #[strum(serialize = "ATTj590587")]
    AttAcsTotalNoOfFlows = 0x902fb,

    #[strum(serialize = "ATTm591136")]
    AttAcsServerList = 0x90520,

    #[strum(serialize = "ATTm590089")]
    AttAdditionalInformation = 0x90109,

    #[strum(serialize = "ATTm590713")]
    AttAdditionalTrustedServiceNames = 0x90379,

    #[strum(serialize = "ATTm131328")]
    AttAddress = 0x20100,

    #[strum(serialize = "ATTb591068")]
    AttAddressBookRoots = 0x904dc,

    #[strum(serialize = "ATTk131396")]
    AttAddressEntryDisplayTable = 0x20144,

    #[strum(serialize = "ATTk131472")]
    AttAddressEntryDisplayTableMsdos = 0x20190,

    #[strum(serialize = "ATTm131689")]
    AttAddressHome = 0x20269,

    #[strum(serialize = "ATTk131327")]
    AttAddressSyntax = 0x200ff,

    #[strum(serialize = "ATTe131422")]
    AttAddressType = 0x2015e,

    #[strum(serialize = "ATTm590438")]
    AttAdminContextMenu = 0x90266,

    #[strum(serialize = "ATTj589974")]
    AttAdminCount = 0x90096,

    #[strum(serialize = "ATTm131298")]
    AttAdminDescription = 0x200e2,

    #[strum(serialize = "ATTm131266")]
    AttAdminDisplayName = 0x200c2,

    #[strum(serialize = "ATTm591514")]
    AttAdminMultiselectPropertyPages = 0x9069a,

    #[strum(serialize = "ATTm590386")]
    AttAdminPropertyPages = 0x90232,

    #[strum(serialize = "ATTc590737")]
    AttAllowedAttributes = 0x90391,

    #[strum(serialize = "ATTc590738")]
    AttAllowedAttributesEffective = 0x90392,

    #[strum(serialize = "ATTc590735")]
    AttAllowedChildClasses = 0x9038f,

    #[strum(serialize = "ATTc590736")]
    AttAllowedChildClassesEffective = 0x90390,

    #[strum(serialize = "ATTm590691")]
    AttAltSecurityIdentities = 0x90363,

    #[strum(serialize = "ATTm591032")]
    AttAnr = 0x904b8,

    #[strum(serialize = "ATTj590672")]
    AttAppSchemaVersion = 0x90350,

    #[strum(serialize = "ATTm590042")]
    AttApplicationName = 0x900da,

    #[strum(serialize = "ATTm590165")]
    AttAppliesTo = 0x90155,

    #[strum(serialize = "ATTm590107")]
    AttAssetNumber = 0x9011b,

    #[strum(serialize = "ATTb590476")]
    AttAssistant = 0x9028c,

    #[strum(serialize = "ATTk591037")]
    AttAssocNtAccount = 0x904bd,

    #[strum(serialize = "ATTk58")]
    AttAttributecertificateattribute = 0x3a,

    #[strum(serialize = "ATTm590572")]
    AttAttributeDisplayNames = 0x902ec,

    #[strum(serialize = "ATTc131102")]
    AttAttributeId = 0x2001e,

    #[strum(serialize = "ATTk589973")]
    AttAttributeSecurityGuid = 0x90095,

    #[strum(serialize = "ATTc131104")]
    AttAttributeSyntax = 0x20020,

    #[strum(serialize = "ATTm1572869")]
    AttAttributeTypes = 0x180005,

    #[strum(serialize = "ATTk1376311")]
    AttAudio = 0x150037,

    #[strum(serialize = "ATTk590026")]
    AttAuditingPolicy = 0x900ca,

    #[strum(serialize = "ATTj589835")]
    AttAuthenticationOptions = 0x9000b,

    #[strum(serialize = "ATTk38")]
    AttAuthorityRevocationList = 0x26,

    #[strum(serialize = "ATTc131423")]
    AttAuxiliaryClass = 0x2015f,

    #[strum(serialize = "ATTq589873")]
    AttBadPasswordTime = 0x90031,

    #[strum(serialize = "ATTj589836")]
    AttBadPwdCount = 0x9000c,

    #[strum(serialize = "ATTk590156")]
    AttBirthLocation = 0x9014c,

    #[strum(serialize = "ATTb590644")]
    AttBridgeheadServerListBl = 0x90334,

    #[strum(serialize = "ATTb590643")]
    AttBridgeheadTransportList = 0x90333,

    #[strum(serialize = "ATTq589837")]
    AttBuiltinCreationTime = 0x9000d,

    #[strum(serialize = "ATTq589838")]
    AttBuiltinModifiedCount = 0x9000e,

    #[strum(serialize = "ATTm15")]
    AttBusinessCategory = 0xf,

    #[strum(serialize = "ATTj590108")]
    AttBytesPerMinute = 0x9011c,

    #[strum(serialize = "ATTk37")]
    AttCaCertificate = 0x25,

    #[strum(serialize = "ATTm590521")]
    AttCaCertificateDn = 0x902b9,

    #[strum(serialize = "ATTm590511")]
    AttCaConnect = 0x902af,

    #[strum(serialize = "ATTm590514")]
    AttCaUsages = 0x902b2,

    #[strum(serialize = "ATTm590512")]
    AttCaWebUrl = 0x902b0,

    #[strum(serialize = "ATTm590639")]
    AttCanUpgradeScript = 0x9032f,

    #[strum(serialize = "ATTm590740")]
    AttCanonicalName = 0x90394,

    #[strum(serialize = "ATTm1441793")]
    AttCarlicense = 0x160001,

    #[strum(serialize = "ATTm590499")]
    AttCatalogs = 0x902a3,

    #[strum(serialize = "ATTm590496")]
    AttCategories = 0x902a0,

    #[strum(serialize = "ATTk590146")]
    AttCategoryId = 0x90142,

    #[strum(serialize = "ATTb590508")]
    AttCertificateAuthorityObject = 0x902ac,

    #[strum(serialize = "ATTk39")]
    AttCertificateRevocationList = 0x27,

    #[strum(serialize = "ATTm590647")]
    AttCertificateTemplates = 0x90337,

    #[strum(serialize = "ATTm590434")]
    AttClassDisplayName = 0x90262,

    #[strum(serialize = "ATTj589840")]
    AttCodePage = 0x90010,

    #[strum(serialize = "ATTm589843")]
    AttComClassid = 0x90013,

    #[strum(serialize = "ATTm590073")]
    AttComClsid = 0x900f9,

    #[strum(serialize = "ATTm589844")]
    AttComInterfaceid = 0x90014,

    #[strum(serialize = "ATTm590077")]
    AttComOtherProgId = 0x900fd,

    #[strum(serialize = "ATTm589845")]
    AttComProgid = 0x90015,

    #[strum(serialize = "ATTm590075")]
    AttComTreatAsClassId = 0x900fb,

    #[strum(serialize = "ATTm590078")]
    AttComTypelibId = 0x900fe,

    #[strum(serialize = "ATTm590074")]
    AttComUniqueLibid = 0x900fa,

    #[strum(serialize = "ATTm131153")]
    AttComment = 0x20051,

    #[strum(serialize = "ATTm3")]
    AttCommonName = 0x3,

    #[strum(serialize = "ATTm131218")]
    AttCompany = 0x20092,

    #[strum(serialize = "ATTi589848")]
    AttContentIndexingAllowed = 0x90018,

    #[strum(serialize = "ATTm590323")]
    AttContextMenu = 0x901f3,

    #[strum(serialize = "ATTk590024")]
    AttControlAccessRights = 0x900c8,

    #[strum(serialize = "ATTj131207")]
    AttCost = 0x20087,

    #[strum(serialize = "ATTj589849")]
    AttCountryCode = 0x90019,

    #[strum(serialize = "ATTm6")]
    AttCountryName = 0x6,

    #[strum(serialize = "ATTm590634")]
    AttCreateDialog = 0x9032a,

    #[strum(serialize = "ATTl1638401")]
    AttCreateTimeStamp = 0x190001,

    #[strum(serialize = "ATTm590636")]
    AttCreateWizardExt = 0x9032c,

    #[strum(serialize = "ATTq589850")]
    AttCreationTime = 0x9001a,

    #[strum(serialize = "ATTm590322")]
    AttCreationWizard = 0x901f2,

    #[strum(serialize = "ATTm590503")]
    AttCreator = 0x902a7,

    #[strum(serialize = "ATTb590513")]
    AttCrlObject = 0x902b1,

    #[strum(serialize = "ATTk590507")]
    AttCrlPartitionedRevocationList = 0x902ab,

    #[strum(serialize = "ATTk40")]
    AttCrossCertificatePair = 0x28,

    #[strum(serialize = "ATTk590161")]
    AttCurrMachineId = 0x90151,

    #[strum(serialize = "ATTk590159")]
    AttCurrentLocation = 0x9014f,

    #[strum(serialize = "ATTb590520")]
    AttCurrentParentCa = 0x902b8,

    #[strum(serialize = "ATTk589851")]
    AttCurrentValue = 0x9001b,

    #[strum(serialize = "ATTk589879")]
    AttDbcsPwd = 0x90037,

    #[strum(serialize = "ATTb590037")]
    AttDefaultClassStore = 0x900d5,

    #[strum(serialize = "ATTb590304")]
    AttDefaultGroup = 0x901e0,

    #[strum(serialize = "ATTi590342")]
    AttDefaultHidingValue = 0x90206,

    #[strum(serialize = "ATTb589881")]
    AttDefaultLocalPolicyObject = 0x90039,

    #[strum(serialize = "ATTb590607")]
    AttDefaultObjectCategory = 0x9030f,

    #[strum(serialize = "ATTj590056")]
    AttDefaultPriority = 0x900e8,

    #[strum(serialize = "ATTm590048")]
    AttDefaultSecurityDescriptor = 0x900e0,

    #[strum(serialize = "ATTk53")]
    AttDeltaRevocationList = 0x35,

    #[strum(serialize = "ATTm131213")]
    AttDepartment = 0x2008d,

    #[strum(serialize = "ATTm1441794")]
    AttDepartmentnumber = 0x160002,

    #[strum(serialize = "ATTm13")]
    AttDescription = 0xd,

    #[strum(serialize = "ATTm590170")]
    AttDesktopProfile = 0x9015a,

    #[strum(serialize = "ATTf27")]
    AttDestinationIndicator = 0x1b,

    #[strum(serialize = "ATTk590539")]
    AttDhcpClasses = 0x902cb,

    #[strum(serialize = "ATTq590524")]
    AttDhcpFlags = 0x902bc,

    #[strum(serialize = "ATTm590525")]
    AttDhcpIdentification = 0x902bd,

    #[strum(serialize = "ATTf590530")]
    AttDhcpMask = 0x902c2,

    #[strum(serialize = "ATTq590543")]
    AttDhcpMaxkey = 0x902cf,

    #[strum(serialize = "ATTm590527")]
    AttDhcpObjDescription = 0x902bf,

    #[strum(serialize = "ATTm590526")]
    AttDhcpObjName = 0x902be,

    #[strum(serialize = "ATTk590538")]
    AttDhcpOptions = 0x902ca,

    #[strum(serialize = "ATTk590542")]
    AttDhcpProperties = 0x902ce,

    #[strum(serialize = "ATTf590531")]
    AttDhcpRanges = 0x902c3,

    #[strum(serialize = "ATTf590533")]
    AttDhcpReservations = 0x902c5,

    #[strum(serialize = "ATTf590528")]
    AttDhcpServers = 0x902c0,

    #[strum(serialize = "ATTf590532")]
    AttDhcpSites = 0x902c4,

    #[strum(serialize = "ATTf590541")]
    AttDhcpState = 0x902cd,

    #[strum(serialize = "ATTf590529")]
    AttDhcpSubnets = 0x902c1,

    #[strum(serialize = "ATTj590523")]
    AttDhcpType = 0x902bb,

    #[strum(serialize = "ATTq590522")]
    AttDhcpUniqueKey = 0x902ba,

    #[strum(serialize = "ATTq590544")]
    AttDhcpUpdateTime = 0x902d0,

    #[strum(serialize = "ATTm131085")]
    AttDisplayName = 0x2000d,

    #[strum(serialize = "ATTf131425")]
    AttDisplayNamePrintable = 0x20161,

    #[strum(serialize = "ATTm1572866")]
    AttDitContentRules = 0x180002,

    #[strum(serialize = "ATTm590085")]
    AttDivision = 0x90105,

    #[strum(serialize = "ATTb131108")]
    AttDmdLocation = 0x20024,

    #[strum(serialize = "ATTm131670")]
    AttDmdName = 0x20256,

    #[strum(serialize = "ATTb591066")]
    AttDnReferenceUpdate = 0x904da,

    #[strum(serialize = "ATTi590202")]
    AttDnsAllowDynamic = 0x9017a,

    #[strum(serialize = "ATTi590203")]
    AttDnsAllowXfr = 0x9017b,

    #[strum(serialize = "ATTm590443")]
    AttDnsHostName = 0x9026b,

    #[strum(serialize = "ATTj590205")]
    AttDnsNotifySecondaries = 0x9017d,

    #[strum(serialize = "ATTk591130")]
    AttDnsProperty = 0x9051a,

    #[strum(serialize = "ATTk590206")]
    AttDnsRecord = 0x9017e,

    #[strum(serialize = "ATTm589852")]
    AttDnsRoot = 0x9001c,

    #[strum(serialize = "ATTj590204")]
    AttDnsSecureSecondaries = 0x9017c,

    #[strum(serialize = "ATTi591238")]
    AttDnsTombstoned = 0x90586,

    #[strum(serialize = "ATTb590492")]
    AttDomainCertificateAuthorities = 0x9029c,

    #[strum(serialize = "ATTm1376281")]
    AttDomainComponent = 0x150019,

    #[strum(serialize = "ATTb590296")]
    AttDomainCrossRef = 0x901d8,

    #[strum(serialize = "ATTb590510")]
    AttDomainId = 0x902ae,

    #[strum(serialize = "ATTj590579")]
    AttDomainIdentifier = 0x902f3,

    #[strum(serialize = "ATTb589856")]
    AttDomainPolicyObject = 0x90020,

    #[strum(serialize = "ATTb590246")]
    AttDomainPolicyReference = 0x901a6,

    #[strum(serialize = "ATTm589982")]
    AttDomainReplica = 0x9009e,

    #[strum(serialize = "ATTk590245")]
    AttDomainWidePolicy = 0x901a5,

    #[strum(serialize = "ATTm590053")]
    AttDriverName = 0x900e5,

    #[strum(serialize = "ATTj590100")]
    AttDriverVersion = 0x90114,

    #[strum(serialize = "ATTl591181")]
    AttDsCorePropagationData = 0x9054d,

    #[strum(serialize = "ATTm131284")]
    AttDsHeuristics = 0x200d4,

    #[strum(serialize = "ATTj591168")]
    AttDsUiAdminMaximum = 0x90540,

    #[strum(serialize = "ATTm591167")]
    AttDsUiAdminNotification = 0x9053f,

    #[strum(serialize = "ATTj591169")]
    AttDsUiShellMaximum = 0x90541,

    #[strum(serialize = "ATTk131146")]
    AttDsaSignature = 0x2004a,

    #[strum(serialize = "ATTb590361")]
    AttDynamicLdapServer = 0x90219,

    #[strum(serialize = "ATTm1376259")]
    AttEMailAddresses = 0x150003,

    #[strum(serialize = "ATTk590092")]
    AttEfspolicy = 0x9010c,

    #[strum(serialize = "ATTm589859")]
    AttEmployeeId = 0x90023,

    #[strum(serialize = "ATTm131682")]
    AttEmployeeNumber = 0x20262,

    #[strum(serialize = "ATTm131685")]
    AttEmployeeType = 0x20265,

    #[strum(serialize = "ATTi131629")]
    AttEnabled = 0x2022d,

    #[strum(serialize = "ATTi589860")]
    AttEnabledConnection = 0x90024,

    #[strum(serialize = "ATTm590649")]
    AttEnrollmentProviders = 0x90339,

    #[strum(serialize = "ATTm590733")]
    AttExtendedAttributeInfo = 0x9038d,

    #[strum(serialize = "ATTi131452")]
    AttExtendedCharsAllowed = 0x2017c,

    #[strum(serialize = "ATTm590732")]
    AttExtendedClassInfo = 0x9038c,

    #[strum(serialize = "ATTm131299")]
    AttExtensionName = 0x200e3,

    #[strum(serialize = "ATTm591511")]
    AttExtraColumns = 0x90697,

    #[strum(serialize = "ATTm23")]
    AttFacsimileTelephoneNumber = 0x17,

    #[strum(serialize = "ATTm590640")]
    AttFileExtPriority = 0x90330,

    #[strum(serialize = "ATTj589862")]
    AttFlags = 0x90026,

    #[strum(serialize = "ATTm590335")]
    AttFlatName = 0x901ff,

    #[strum(serialize = "ATTq589863")]
    AttForceLogoff = 0x90027,

    #[strum(serialize = "ATTk590180")]
    AttForeignIdentifier = 0x90164,

    #[strum(serialize = "ATTm590506")]
    AttFriendlyNames = 0x902aa,

    #[strum(serialize = "ATTi590734")]
    AttFromEntry = 0x9038e,

    #[strum(serialize = "ATTb589864")]
    AttFromServer = 0x90028,

    #[strum(serialize = "ATTb590693")]
    AttFrsComputerReference = 0x90365,

    #[strum(serialize = "ATTb590694")]
    AttFrsComputerReferenceBl = 0x90366,

    #[strum(serialize = "ATTm590695")]
    AttFrsControlDataCreation = 0x90367,

    #[strum(serialize = "ATTm590696")]
    AttFrsControlInboundBacklog = 0x90368,

    #[strum(serialize = "ATTm590697")]
    AttFrsControlOutboundBacklog = 0x90369,

    #[strum(serialize = "ATTm590308")]
    AttFrsDirectoryFilter = 0x901e4,

    #[strum(serialize = "ATTj590314")]
    AttFrsDsPoll = 0x901ea,

    #[strum(serialize = "ATTk590360")]
    AttFrsExtensions = 0x90218,

    #[strum(serialize = "ATTm590315")]
    AttFrsFaultCondition = 0x901eb,

    #[strum(serialize = "ATTm590307")]
    AttFrsFileFilter = 0x901e3,

    #[strum(serialize = "ATTj590698")]
    AttFrsFlags = 0x9036a,

    #[strum(serialize = "ATTj590358")]
    AttFrsLevelLimit = 0x90216,

    #[strum(serialize = "ATTb590699")]
    AttFrsMemberReference = 0x9036b,

    #[strum(serialize = "ATTb590700")]
    AttFrsMemberReferenceBl = 0x9036c,

    #[strum(serialize = "ATTj590701")]
    AttFrsPartnerAuthLevel = 0x9036d,

    #[strum(serialize = "ATTb590702")]
    AttFrsPrimaryMember = 0x9036e,

    #[strum(serialize = "ATTk590357")]
    AttFrsReplicaSetGuid = 0x90215,

    #[strum(serialize = "ATTj589855")]
    AttFrsReplicaSetType = 0x9001f,

    #[strum(serialize = "ATTm590311")]
    AttFrsRootPath = 0x901e7,

    #[strum(serialize = "ATTp590359")]
    AttFrsRootSecurity = 0x90217,

    #[strum(serialize = "ATTm590324")]
    AttFrsServiceCommand = 0x901f4,

    #[strum(serialize = "ATTm590703")]
    AttFrsServiceCommandStatus = 0x9036f,

    #[strum(serialize = "ATTm590312")]
    AttFrsStagingPath = 0x901e8,

    #[strum(serialize = "ATTl590704")]
    AttFrsTimeLastCommand = 0x90370,

    #[strum(serialize = "ATTl590705")]
    AttFrsTimeLastConfigChange = 0x90371,

    #[strum(serialize = "ATTj590309")]
    AttFrsUpdateTimeout = 0x901e5,

    #[strum(serialize = "ATTm590706")]
    AttFrsVersion = 0x90372,

    #[strum(serialize = "ATTk589867")]
    AttFrsVersionGuid = 0x9002b,

    #[strum(serialize = "ATTm590310")]
    AttFrsWorkingPath = 0x901e6,

    #[strum(serialize = "ATTb590193")]
    AttFsmoRoleOwner = 0x90171,

    #[strum(serialize = "ATTj131373")]
    AttGarbageCollPeriod = 0x2012d,

    #[strum(serialize = "ATTi589865")]
    AttGeneratedConnection = 0x90029,

    #[strum(serialize = "ATTm44")]
    AttGenerationQualifier = 0x2c,

    #[strum(serialize = "ATTm42")]
    AttGivenName = 0x2a,

    #[strum(serialize = "ATTb591069")]
    AttGlobalAddressList = 0x904dd,

    #[strum(serialize = "ATTc131094")]
    AttGovernsId = 0x20016,

    #[strum(serialize = "ATTm590715")]
    AttGpLink = 0x9037b,

    #[strum(serialize = "ATTj590716")]
    AttGpOptions = 0x9037c,

    #[strum(serialize = "ATTm590718")]
    AttGpcFileSysPath = 0x9037e,

    #[strum(serialize = "ATTj590717")]
    AttGpcFunctionalityVersion = 0x9037d,

    #[strum(serialize = "ATTm591172")]
    AttGpcMachineExtensionNames = 0x90544,

    #[strum(serialize = "ATTm591173")]
    AttGpcUserExtensionNames = 0x90545,

    #[strum(serialize = "ATTm591518")]
    AttGpcWqlFilter = 0x9069e,

    #[strum(serialize = "ATTj589976")]
    AttGroupAttributes = 0x90098,

    #[strum(serialize = "ATTk589990")]
    AttGroupMembershipSam = 0x900a6,

    #[strum(serialize = "ATTm590169")]
    AttGroupPriority = 0x90159,

    #[strum(serialize = "ATTj590574")]
    AttGroupType = 0x902ee,

    #[strum(serialize = "ATTm590168")]
    AttGroupsToIgnore = 0x90158,

    #[strum(serialize = "ATTb131086")]
    AttHasMasterNcs = 0x2000e,

    #[strum(serialize = "ATTb131087")]
    AttHasPartialReplicaNcs = 0x2000f,

    #[strum(serialize = "ATTk131474")]
    AttHelpData16 = 0x20192,

    #[strum(serialize = "ATTk131081")]
    AttHelpData32 = 0x20009,

    #[strum(serialize = "ATTm131399")]
    AttHelpFileName = 0x20147,

    #[strum(serialize = "ATTi591604")]
    AttHideFromAb = 0x906f4,

    #[strum(serialize = "ATTm589868")]
    AttHomeDirectory = 0x9002c,

    #[strum(serialize = "ATTm589869")]
    AttHomeDrive = 0x9002d,

    #[strum(serialize = "ATTm590043")]
    AttIconPath = 0x900db,

    #[strum(serialize = "ATTk590144")]
    AttImplementedCategories = 0x90140,

    #[strum(serialize = "ATTm590505")]
    AttIndexedscopes = 0x902a9,

    #[strum(serialize = "ATTm590363")]
    AttInitialAuthIncoming = 0x9021b,

    #[strum(serialize = "ATTm590364")]
    AttInitialAuthOutgoing = 0x9021c,

    #[strum(serialize = "ATTm43")]
    AttInitials = 0x2b,

    #[strum(serialize = "ATTj590671")]
    AttInstallUiLevel = 0x9034f,

    #[strum(serialize = "ATTj131073")]
    AttInstanceType = 0x20001,

    #[strum(serialize = "ATTj591072")]
    AttInterSiteTopologyFailover = 0x904e0,

    #[strum(serialize = "ATTb591070")]
    AttInterSiteTopologyGenerator = 0x904de,

    #[strum(serialize = "ATTj591071")]
    AttInterSiteTopologyRenew = 0x904df,

    #[strum(serialize = "ATTg25")]
    AttInternationalIsdnNumber = 0x19,

    #[strum(serialize = "ATTk131187")]
    AttInvocationId = 0x20073,

    #[strum(serialize = "ATTk590447")]
    AttIpsecData = 0x9026f,

    #[strum(serialize = "ATTj590446")]
    AttIpsecDataType = 0x9026e,

    #[strum(serialize = "ATTb590453")]
    AttIpsecFilterReference = 0x90275,

    #[strum(serialize = "ATTm590445")]
    AttIpsecId = 0x9026d,

    #[strum(serialize = "ATTb590450")]
    AttIpsecIsakmpReference = 0x90272,

    #[strum(serialize = "ATTm590444")]
    AttIpsecName = 0x9026c,

    #[strum(serialize = "ATTm590712")]
    AttIpsecNegotiationPolicyAction = 0x90378,

    #[strum(serialize = "ATTb590452")]
    AttIpsecNegotiationPolicyReference = 0x90274,

    #[strum(serialize = "ATTm590711")]
    AttIpsecNegotiationPolicyType = 0x90377,

    #[strum(serialize = "ATTb590451")]
    AttIpsecNfaReference = 0x90273,

    #[strum(serialize = "ATTb590448")]
    AttIpsecOwnersReference = 0x90270,

    #[strum(serialize = "ATTb590341")]
    AttIpsecPolicyReference = 0x90205,

    #[strum(serialize = "ATTi590692")]
    AttIsCriticalSystemObject = 0x90364,

    #[strum(serialize = "ATTi590485")]
    AttIsDefunct = 0x90295,

    #[strum(serialize = "ATTi131120")]
    AttIsDeleted = 0x20030,

    #[strum(serialize = "ATTi591036")]
    AttIsEphemeral = 0x904bc,

    #[strum(serialize = "ATTb131174")]
    AttIsMemberOfDl = 0x20066,

    #[strum(serialize = "ATTi590463")]
    AttIsMemberOfPartialAttributeSet = 0x9027f,

    #[strum(serialize = "ATTb590462")]
    AttIsPrivilegeHolder = 0x9027e,

    #[strum(serialize = "ATTi131105")]
    AttIsSingleValued = 0x20021,

    #[strum(serialize = "ATTk1376316")]
    AttJpegphoto = 0x15003c,

    #[strum(serialize = "ATTm589872")]
    AttKeywords = 0x90030,

    #[strum(serialize = "ATTe2")]
    AttKnowledgeInformation = 0x2,

    #[strum(serialize = "ATTq590343")]
    AttLastBackupRestorationTime = 0x90207,

    #[strum(serialize = "ATTq589874")]
    AttLastContentIndexed = 0x90032,

    #[strum(serialize = "ATTb590605")]
    AttLastKnownParent = 0x9030d,

    #[strum(serialize = "ATTq589875")]
    AttLastLogoff = 0x90033,

    #[strum(serialize = "ATTq589876")]
    AttLastLogon = 0x90034,

    #[strum(serialize = "ATTq591520")]
    AttLastLogonTimestamp = 0x906a0,

    #[strum(serialize = "ATTq589877")]
    AttLastSetTime = 0x90035,

    #[strum(serialize = "ATTm590154")]
    AttLastUpdateSequence = 0x9014a,

    #[strum(serialize = "ATTm590667")]
    AttLdapAdminLimits = 0x9034b,

    #[strum(serialize = "ATTm131532")]
    AttLdapDisplayName = 0x201cc,

    #[strum(serialize = "ATTk590668")]
    AttLdapIpdenyList = 0x9034c,

    #[strum(serialize = "ATTe590479")]
    AttLegacyExchangeDn = 0x9028f,

    #[strum(serialize = "ATTj131122")]
    AttLinkId = 0x20032,

    #[strum(serialize = "ATTk590093")]
    AttLinkTrackSecret = 0x9010d,

    #[strum(serialize = "ATTk589984")]
    AttLmPwdHistory = 0x900a0,

    #[strum(serialize = "ATTj589880")]
    AttLocalPolicyFlags = 0x90038,

    #[strum(serialize = "ATTb590281")]
    AttLocalPolicyReference = 0x901c9,

    #[strum(serialize = "ATTj589882")]
    AttLocaleId = 0x9003a,

    #[strum(serialize = "ATTm7")]
    AttLocalityName = 0x7,

    #[strum(serialize = "ATTm590641")]
    AttLocalizedDescription = 0x90331,

    #[strum(serialize = "ATTj591177")]
    AttLocalizationDisplayId = 0x90549,

    #[strum(serialize = "ATTm590046")]
    AttLocation = 0x900de,

    #[strum(serialize = "ATTq589885")]
    AttLockOutObservationWindow = 0x9003d,

    #[strum(serialize = "ATTq589884")]
    AttLockoutDuration = 0x9003c,

    #[strum(serialize = "ATTj589897")]
    AttLockoutThreshold = 0x90049,

    #[strum(serialize = "ATTq590486")]
    AttLockoutTime = 0x90296,

    #[strum(serialize = "ATTk1441828")]
    AttLogo = 0x160024,

    #[strum(serialize = "ATTj589993")]
    AttLogonCount = 0x900a9,

    #[strum(serialize = "ATTk589888")]
    AttLogonHours = 0x90040,

    #[strum(serialize = "ATTk589889")]
    AttLogonWorkstation = 0x90041,

    #[strum(serialize = "ATTq589890")]
    AttLsaCreationTime = 0x90042,

    #[strum(serialize = "ATTq589891")]
    AttLsaModifiedCount = 0x90043,

    #[strum(serialize = "ATTj589892")]
    AttMachineArchitecture = 0x90044,

    #[strum(serialize = "ATTq590344")]
    AttMachinePasswordChangeInterval = 0x90208,

    #[strum(serialize = "ATTj589895")]
    AttMachineRole = 0x90047,

    #[strum(serialize = "ATTk590283")]
    AttMachineWidePolicy = 0x901cb,

    #[strum(serialize = "ATTb590477")]
    AttManagedBy = 0x9028d,

    #[strum(serialize = "ATTb590478")]
    AttManagedObjects = 0x9028e,

    #[strum(serialize = "ATTb1376266")]
    AttManager = 0x15000a,

    #[strum(serialize = "ATTj131121")]
    AttMapiId = 0x20031,

    #[strum(serialize = "ATTk589896")]
    AttMarshalledInterface = 0x90048,

    #[strum(serialize = "ATTb591233")]
    AttMasteredBy = 0x90581,

    #[strum(serialize = "ATTq589898")]
    AttMaxPwdAge = 0x9004a,

    #[strum(serialize = "ATTq589899")]
    AttMaxRenewAge = 0x9004b,

    #[strum(serialize = "ATTq589900")]
    AttMaxStorage = 0x9004c,

    #[strum(serialize = "ATTq589901")]
    AttMaxTicketAge = 0x9004d,

    #[strum(serialize = "ATTc131097")]
    AttMayContain = 0x20019,

    #[strum(serialize = "ATTm590406")]
    AttMeetingadvertisescope = 0x90246,

    #[strum(serialize = "ATTm590397")]
    AttMeetingapplication = 0x9023d,

    #[strum(serialize = "ATTj590413")]
    AttMeetingbandwidth = 0x9024d,

    #[strum(serialize = "ATTk590414")]
    AttMeetingblob = 0x9024e,

    #[strum(serialize = "ATTm590402")]
    AttMeetingcontactinfo = 0x90242,

    #[strum(serialize = "ATTm590391")]
    AttMeetingdescription = 0x90237,

    #[strum(serialize = "ATTl590412")]
    AttMeetingendtime = 0x9024c,

    #[strum(serialize = "ATTm590389")]
    AttMeetingid = 0x90235,

    #[strum(serialize = "ATTm590404")]
    AttMeetingip = 0x90244,

    #[strum(serialize = "ATTm590409")]
    AttMeetingisencrypted = 0x90249,

    #[strum(serialize = "ATTm590392")]
    AttMeetingkeyword = 0x90238,

    #[strum(serialize = "ATTm590398")]
    AttMeetinglanguage = 0x9023e,

    #[strum(serialize = "ATTm590393")]
    AttMeetinglocation = 0x90239,

    #[strum(serialize = "ATTj590400")]
    AttMeetingmaxparticipants = 0x90240,

    #[strum(serialize = "ATTm590390")]
    AttMeetingname = 0x90236,

    #[strum(serialize = "ATTm590401")]
    AttMeetingoriginator = 0x90241,

    #[strum(serialize = "ATTm590403")]
    AttMeetingowner = 0x90243,

    #[strum(serialize = "ATTm590394")]
    AttMeetingprotocol = 0x9023a,

    #[strum(serialize = "ATTm590408")]
    AttMeetingrating = 0x90248,

    #[strum(serialize = "ATTm590410")]
    AttMeetingrecurrence = 0x9024a,

    #[strum(serialize = "ATTm590405")]
    AttMeetingscope = 0x90245,

    #[strum(serialize = "ATTl590411")]
    AttMeetingstarttime = 0x9024b,

    #[strum(serialize = "ATTm590395")]
    AttMeetingtype = 0x9023b,

    #[strum(serialize = "ATTm590407")]
    AttMeetingurl = 0x90247,

    #[strum(serialize = "ATTb31")]
    AttMember = 0x1f,

    #[strum(serialize = "ATTm590474")]
    AttMhsOrAddress = 0x9028a,

    #[strum(serialize = "ATTq589902")]
    AttMinPwdAge = 0x9004e,

    #[strum(serialize = "ATTj589903")]
    AttMinPwdLength = 0x9004f,

    #[strum(serialize = "ATTq589904")]
    AttMinTicketAge = 0x90050,

    #[strum(serialize = "ATTq589992")]
    AttModifiedCount = 0x900a8,

    #[strum(serialize = "ATTq589905")]
    AttModifiedCountAtLastProm = 0x90051,

    #[strum(serialize = "ATTl1638402")]
    AttModifyTimeStamp = 0x190002,

    #[strum(serialize = "ATTk589906")]
    AttMoniker = 0x90052,

    #[strum(serialize = "ATTm589907")]
    AttMonikerDisplayName = 0x90053,

    #[strum(serialize = "ATTk591129")]
    AttMoveTreeState = 0x90519,

    #[strum(serialize = "ATTb591251")]
    AttMsComDefaultpartitionlink = 0x90593,

    #[strum(serialize = "ATTk591252")]
    AttMsComObjectid = 0x90594,

    #[strum(serialize = "ATTb591247")]
    AttMsComPartitionlink = 0x9058f,

    #[strum(serialize = "ATTb591248")]
    AttMsComPartitionsetlink = 0x90590,

    #[strum(serialize = "ATTb591249")]
    AttMsComUserlink = 0x90591,

    #[strum(serialize = "ATTb591250")]
    AttMsComUserpartitionsetlink = 0x90592,

    #[strum(serialize = "ATTm591541")]
    AttMsDsAdditionalDnsHostName = 0x906b5,

    #[strum(serialize = "ATTm591542")]
    AttMsDsAdditionalSamAccountName = 0x906b6,

    #[strum(serialize = "ATTj591613")]
    AttMsDsAllUsersTrustQuota = 0x906fd,

    #[strum(serialize = "ATTm591534")]
    AttMsDsAllowedDnsSuffixes = 0x906ae,

    #[strum(serialize = "ATTm591611")]
    AttMsDsAllowedToDelegateTo = 0x906fb,

    #[strum(serialize = "ATTc591282")]
    AttMsDsAuxiliaryClasses = 0x905b2,

    #[strum(serialize = "ATTj591493")]
    AttMsDsApproxImmedSubordinates = 0x90685,

    #[strum(serialize = "ATTj591283")]
    AttMsDsBehaviorVersion = 0x905b3,

    #[strum(serialize = "ATTk591265")]
    AttMsDsCachedMembership = 0x905a1,

    #[strum(serialize = "ATTq591266")]
    AttMsDsCachedMembershipTimeStamp = 0x905a2,

    #[strum(serialize = "ATTk591184")]
    AttMsDsConsistencyGuid = 0x90550,

    #[strum(serialize = "ATTj591185")]
    AttMsDsConsistencyChildCount = 0x90551,

    #[strum(serialize = "ATTr591234")]
    AttMsDsCreatorSid = 0x90582,

    #[strum(serialize = "ATTm591543")]
    AttMsDsDnsrootalias = 0x906b7,

    #[strum(serialize = "ATTl591446")]
    AttMsDsEntryTimeToDie = 0x90656,

    #[strum(serialize = "ATTk591607")]
    AttMsDsExecutescriptpassword = 0x906f7,

    #[strum(serialize = "ATTm591527")]
    AttMsDsFilterContainers = 0x906a7,

    #[strum(serialize = "ATTh591533")]
    AttMsDsHasInstantiatedNcs = 0x906ad,

    #[strum(serialize = "ATTj591540")]
    AttMsDsIntid = 0x906b4,

    #[strum(serialize = "ATTj591608")]
    AttMsDsLogonTimeSyncInterval = 0x906f8,

    #[strum(serialize = "ATTk591526")]
    AttMsDsTrustForestTrustInfo = 0x906a6,

    #[strum(serialize = "ATTj591235")]
    AttMsDsMachineAccountQuota = 0x90583,

    #[strum(serialize = "ATTm591445")]
    AttMsDsOtherSettings = 0x90655,

    #[strum(serialize = "ATTm591528")]
    AttMsDsNcReplCursors = 0x906a8,

    #[strum(serialize = "ATTm591529")]
    AttMsDsNcReplInboundNeighbors = 0x906a9,

    #[strum(serialize = "ATTm591530")]
    AttMsDsNcReplOutboundNeighbors = 0x906aa,

    #[strum(serialize = "ATTb591485")]
    AttMsDsNcReplicaLocations = 0x9067d,

    #[strum(serialize = "ATTm591513")]
    AttMsDsNonSecurityGroupExtraClasses = 0x90699,

    #[strum(serialize = "ATTj591612")]
    AttMsDsPerUserTrustQuota = 0x906fc,

    #[strum(serialize = "ATTj591614")]
    AttMsDsPerUserTrustTombstonesQuota = 0x906fe,

    #[strum(serialize = "ATTb591268")]
    AttMsDsPreferredGcSite = 0x905a4,

    #[strum(serialize = "ATTm591531")]
    AttMsDsReplAttributeMetaData = 0x906ab,

    #[strum(serialize = "ATTm591532")]
    AttMsDsReplValueMetaData = 0x906ac,

    #[strum(serialize = "ATTh591232")]
    AttMsDsReplicatesNcReason = 0x90580,

    #[strum(serialize = "ATTj591487")]
    AttMsDsReplicationNotifyFirstDsaDelay = 0x9067f,

    #[strum(serialize = "ATTj591488")]
    AttMsDsReplicationNotifySubsequentDsaDelay = 0x90680,

    #[strum(serialize = "ATTj591544")]
    AttMsDsReplicationepoch = 0x906b8,

    #[strum(serialize = "ATTk591264")]
    AttMsDsSchemaExtensions = 0x905a0,

    #[strum(serialize = "ATTb591535")]
    AttMsDsSdReferenceDomain = 0x906af,

    #[strum(serialize = "ATTm591512")]
    AttMsDsSecurityGroupExtraClasses = 0x90698,

    #[strum(serialize = "ATTm591521")]
    AttMsDsSettings = 0x906a1,

    #[strum(serialize = "ATTk591267")]
    AttMsDsSiteAffinity = 0x905a3,

    #[strum(serialize = "ATTm591539")]
    AttMsDsSpnSuffixes = 0x906b3,

    #[strum(serialize = "ATTj591284")]
    AttMsDsUserAccountControlComputed = 0x905b4,

    #[strum(serialize = "ATTm591545")]
    AttMsDsUpdatescript = 0x906b9,

    #[strum(serialize = "ATTm131516")]
    AttMsExchAssistantName = 0x201bc,

    #[strum(serialize = "ATTm131665")]
    AttMsExchLabeleduri = 0x20251,

    #[strum(serialize = "ATTb131176")]
    AttMsExchOwnerBl = 0x20068,

    #[strum(serialize = "ATTb591517")]
    AttMsFrsHubMember = 0x9069d,

    #[strum(serialize = "ATTm591516")]
    AttMsFrsTopologyPref = 0x9069c,

    #[strum(serialize = "ATTm591610")]
    AttMsIisFtpDir = 0x906fa,

    #[strum(serialize = "ATTm591609")]
    AttMsIisFtpRoot = 0x906f9,

    #[strum(serialize = "ATTk591548")]
    AttMsMmsData = 0x906bc,

    #[strum(serialize = "ATTk591549")]
    AttMsMmsIndex = 0x906bd,

    #[strum(serialize = "ATTk591550")]
    AttMsMmsIndice = 0x906be,

    #[strum(serialize = "ATTm591551")]
    AttMsMmsXml = 0x906bf,

    #[strum(serialize = "ATTb591552")]
    AttMsMmsJoinLink = 0x906c0,

    #[strum(serialize = "ATTk591553")]
    AttMsMmsLineage = 0x906c1,

    #[strum(serialize = "ATTm591554")]
    AttMsMmsProvStatus = 0x906c2,

    #[strum(serialize = "ATTk591555")]
    AttMsMmsSyncStatus = 0x906c3,

    #[strum(serialize = "ATTm591556")]
    AttMsMmsPartition = 0x906c4,

    #[strum(serialize = "ATTb591557")]
    AttMsMmsMaStagingLink = 0x906c5,

    #[strum(serialize = "ATTb591558")]
    AttMsMmsMaStagingBl = 0x906c6,

    #[strum(serialize = "ATTb591559")]
    AttMsMmsProvisioningLink = 0x906c7,

    #[strum(serialize = "ATTb591560")]
    AttMsMmsProvisioningBl = 0x906c8,

    #[strum(serialize = "ATTb591561")]
    AttMsMmsAssociatedLink = 0x906c9,

    #[strum(serialize = "ATTb591562")]
    AttMsMmsAssociatedBl = 0x906ca,

    #[strum(serialize = "ATTb591563")]
    AttMsMmsScopeLink = 0x906cb,

    #[strum(serialize = "ATTb591564")]
    AttMsMmsScopeBl = 0x906cc,

    #[strum(serialize = "ATTm591565")]
    AttMsMmsCriteria = 0x906cd,

    #[strum(serialize = "ATTb591566")]
    AttMsMmsDomainController = 0x906ce,

    #[strum(serialize = "ATTm591567")]
    AttMsMmsServiceName = 0x906cf,

    #[strum(serialize = "ATTm591568")]
    AttMsMmsInstanceConfiguration = 0x906d0,

    #[strum(serialize = "ATTm591569")]
    AttMsMmsToolsConfiguration = 0x906d1,

    #[strum(serialize = "ATTm591570")]
    AttMsMmsInstanceRule = 0x906d2,

    #[strum(serialize = "ATTm591571")]
    AttMsMmsInstallStatus = 0x906d3,

    #[strum(serialize = "ATTm591572")]
    AttMsMmsVersion = 0x906d4,

    #[strum(serialize = "ATTb591573")]
    AttMsMmsConnectorSpace = 0x906d5,

    #[strum(serialize = "ATTb591574")]
    AttMsMmsScope = 0x906d6,

    #[strum(serialize = "ATTm591575")]
    AttMsMmsInstanceSchedule = 0x906d7,

    #[strum(serialize = "ATTk591576")]
    AttMsMmsInstanceInfo = 0x906d8,

    #[strum(serialize = "ATTm591577")]
    AttMsMmsMaConfiguration = 0x906d9,

    #[strum(serialize = "ATTk591578")]
    AttMsMmsMaConfigurationPrivate = 0x906da,

    #[strum(serialize = "ATTm591579")]
    AttMsMmsMaSchema = 0x906db,

    #[strum(serialize = "ATTm591580")]
    AttMsMmsMaMap = 0x906dc,

    #[strum(serialize = "ATTm591581")]
    AttMsMmsMaCapability = 0x906dd,

    #[strum(serialize = "ATTm591582")]
    AttMsMmsMaExecutionHistory = 0x906de,

    #[strum(serialize = "ATTm591583")]
    AttMsMmsMaCategory = 0x906df,

    #[strum(serialize = "ATTk591584")]
    AttMsMmsMaAdInfo = 0x906e0,

    #[strum(serialize = "ATTk591585")]
    AttMsMmsMaCdInfo = 0x906e1,

    #[strum(serialize = "ATTk591586")]
    AttMsMmsMaProcessInfo = 0x906e2,

    #[strum(serialize = "ATTk591587")]
    AttMsMmsMaScriptInfo = 0x906e3,

    #[strum(serialize = "ATTk591588")]
    AttMsMmsMaSystem = 0x906e4,

    #[strum(serialize = "ATTk591589")]
    AttMsMmsMaSynchronization = 0x906e5,

    #[strum(serialize = "ATTb591590")]
    AttMsMmsJoinBl = 0x906e6,

    #[strum(serialize = "ATTk591591")]
    AttMsMmsAnchor = 0x906e7,

    #[strum(serialize = "ATTk591592")]
    AttMsMmsExportKey = 0x906e8,

    #[strum(serialize = "ATTk591593")]
    AttMsMmsImportKey = 0x906e9,

    #[strum(serialize = "ATTk591594")]
    AttMsMmsState = 0x906ea,

    #[strum(serialize = "ATTk591595")]
    AttMsMmsHologram = 0x906eb,

    #[strum(serialize = "ATTk591596")]
    AttMsMmsDeltaHologram = 0x906ec,

    #[strum(serialize = "ATTm591597")]
    AttMsMmsProvisioningConfiguration = 0x906ed,

    #[strum(serialize = "ATTk591598")]
    AttMsMmsProvisioningConfigurationPrivate = 0x906ee,

    #[strum(serialize = "ATTk591599")]
    AttMsMmsProvisioningAdInfo = 0x906ef,

    #[strum(serialize = "ATTm591600")]
    AttMsMmsProvisioningSystem = 0x906f0,

    #[strum(serialize = "ATTm591601")]
    AttMsMmsProvisioningStatusXml = 0x906f1,

    #[strum(serialize = "ATTk591602")]
    AttMsMmsProvisioningStatusBinary = 0x906f2,

    #[strum(serialize = "ATTm591260")]
    AttMsPkiCertTemplateOid = 0x9059c,

    #[strum(serialize = "ATTm591498")]
    AttMsPkiCertificateApplicationPolicy = 0x9068a,

    #[strum(serialize = "ATTj591256")]
    AttMsPkiCertificateNameFlag = 0x90598,

    #[strum(serialize = "ATTm591263")]
    AttMsPkiCertificatePolicy = 0x9059f,

    #[strum(serialize = "ATTj591254")]
    AttMsPkiEnrollmentFlag = 0x90596,

    #[strum(serialize = "ATTj591257")]
    AttMsPkiMinimalKeySize = 0x90599,

    #[strum(serialize = "ATTj591495")]
    AttMsPkiOidAttribute = 0x90687,

    #[strum(serialize = "ATTm591496")]
    AttMsPkiOidCps = 0x90688,

    #[strum(serialize = "ATTm591536")]
    AttMsPkiOidLocalizedname = 0x906b0,

    #[strum(serialize = "ATTm591497")]
    AttMsPkiOidUserNotice = 0x90689,

    #[strum(serialize = "ATTj591255")]
    AttMsPkiPrivateKeyFlag = 0x90597,

    #[strum(serialize = "ATTm591261")]
    AttMsPkiSupersedeTemplates = 0x9059d,

    #[strum(serialize = "ATTj591259")]
    AttMsPkiTemplateMinorRevision = 0x9059b,

    #[strum(serialize = "ATTj591258")]
    AttMsPkiTemplateSchemaVersion = 0x9059a,

    #[strum(serialize = "ATTm591499")]
    AttMsPkiRaApplicationPolicies = 0x9068b,

    #[strum(serialize = "ATTm591262")]
    AttMsPkiRaPolicies = 0x9059e,

    #[strum(serialize = "ATTj591253")]
    AttMsPkiRaSignature = 0x90595,

    #[strum(serialize = "ATTm590708")]
    AttMsRrasAttribute = 0x90374,

    #[strum(serialize = "ATTm590707")]
    AttMsRrasVendorAttributeEntry = 0x90373,

    #[strum(serialize = "ATTm591187")]
    AttMsSqlName = 0x90553,

    #[strum(serialize = "ATTm591188")]
    AttMsSqlRegisteredowner = 0x90554,

    #[strum(serialize = "ATTm591189")]
    AttMsSqlContact = 0x90555,

    #[strum(serialize = "ATTm591190")]
    AttMsSqlLocation = 0x90556,

    #[strum(serialize = "ATTq591191")]
    AttMsSqlMemory = 0x90557,

    #[strum(serialize = "ATTj591192")]
    AttMsSqlBuild = 0x90558,

    #[strum(serialize = "ATTm591193")]
    AttMsSqlServiceaccount = 0x90559,

    #[strum(serialize = "ATTj591194")]
    AttMsSqlCharacterset = 0x9055a,

    #[strum(serialize = "ATTm591195")]
    AttMsSqlSortorder = 0x9055b,

    #[strum(serialize = "ATTj591196")]
    AttMsSqlUnicodesortorder = 0x9055c,

    #[strum(serialize = "ATTi591197")]
    AttMsSqlClustered = 0x9055d,

    #[strum(serialize = "ATTm591198")]
    AttMsSqlNamedpipe = 0x9055e,

    #[strum(serialize = "ATTm591199")]
    AttMsSqlMultiprotocol = 0x9055f,

    #[strum(serialize = "ATTm591200")]
    AttMsSqlSpx = 0x90560,

    #[strum(serialize = "ATTm591201")]
    AttMsSqlTcpip = 0x90561,

    #[strum(serialize = "ATTm591202")]
    AttMsSqlAppletalk = 0x90562,

    #[strum(serialize = "ATTm591203")]
    AttMsSqlVines = 0x90563,

    #[strum(serialize = "ATTq591204")]
    AttMsSqlStatus = 0x90564,

    #[strum(serialize = "ATTm591205")]
    AttMsSqlLastupdateddate = 0x90565,

    #[strum(serialize = "ATTm591206")]
    AttMsSqlInformationurl = 0x90566,

    #[strum(serialize = "ATTm591207")]
    AttMsSqlConnectionurl = 0x90567,

    #[strum(serialize = "ATTm591208")]
    AttMsSqlPublicationurl = 0x90568,

    #[strum(serialize = "ATTm591209")]
    AttMsSqlGpslatitude = 0x90569,

    #[strum(serialize = "ATTm591210")]
    AttMsSqlGpslongitude = 0x9056a,

    #[strum(serialize = "ATTm591211")]
    AttMsSqlGpsheight = 0x9056b,

    #[strum(serialize = "ATTm591212")]
    AttMsSqlVersion = 0x9056c,

    #[strum(serialize = "ATTm591213")]
    AttMsSqlLanguage = 0x9056d,

    #[strum(serialize = "ATTm591214")]
    AttMsSqlDescription = 0x9056e,

    #[strum(serialize = "ATTm591215")]
    AttMsSqlType = 0x9056f,

    #[strum(serialize = "ATTi591216")]
    AttMsSqlInformationdirectory = 0x90570,

    #[strum(serialize = "ATTm591217")]
    AttMsSqlDatabase = 0x90571,

    #[strum(serialize = "ATTi591218")]
    AttMsSqlAllowanonymoussubscription = 0x90572,

    #[strum(serialize = "ATTm591219")]
    AttMsSqlAlias = 0x90573,

    #[strum(serialize = "ATTq591220")]
    AttMsSqlSize = 0x90574,

    #[strum(serialize = "ATTm591221")]
    AttMsSqlCreationdate = 0x90575,

    #[strum(serialize = "ATTm591222")]
    AttMsSqlLastbackupdate = 0x90576,

    #[strum(serialize = "ATTm591223")]
    AttMsSqlLastdiagnosticdate = 0x90577,

    #[strum(serialize = "ATTm591224")]
    AttMsSqlApplications = 0x90578,

    #[strum(serialize = "ATTm591225")]
    AttMsSqlKeywords = 0x90579,

    #[strum(serialize = "ATTm591226")]
    AttMsSqlPublisher = 0x9057a,

    #[strum(serialize = "ATTi591227")]
    AttMsSqlAllowknownpullsubscription = 0x9057b,

    #[strum(serialize = "ATTi591228")]
    AttMsSqlAllowimmediateupdatingsubscription = 0x9057c,

    #[strum(serialize = "ATTi591229")]
    AttMsSqlAllowqueuedupdatingsubscription = 0x9057d,

    #[strum(serialize = "ATTi591230")]
    AttMsSqlAllowsnapshotfilesftpdownloading = 0x9057e,

    #[strum(serialize = "ATTi591231")]
    AttMsSqlThirdparty = 0x9057f,

    #[strum(serialize = "ATTk591524")]
    AttMsTapiConferenceBlob = 0x906a4,

    #[strum(serialize = "ATTm591525")]
    AttMsTapiIpAddress = 0x906a5,

    #[strum(serialize = "ATTm591523")]
    AttMsTapiProtocolId = 0x906a3,

    #[strum(serialize = "ATTm591522")]
    AttMsTapiUniqueIdentifier = 0x906a2,

    #[strum(serialize = "ATTm591447")]
    AttMsWmiAuthor = 0x90657,

    #[strum(serialize = "ATTm591448")]
    AttMsWmiChangedate = 0x90658,

    #[strum(serialize = "ATTm591500")]
    AttMsWmiClass = 0x9068c,

    #[strum(serialize = "ATTm591449")]
    AttMsWmiClassdefinition = 0x90659,

    #[strum(serialize = "ATTm591450")]
    AttMsWmiCreationdate = 0x9065a,

    #[strum(serialize = "ATTj591501")]
    AttMsWmiGenus = 0x9068d,

    #[strum(serialize = "ATTm591451")]
    AttMsWmiId = 0x9065b,

    #[strum(serialize = "ATTj591452")]
    AttMsWmiIntdefault = 0x9065c,

    #[strum(serialize = "ATTj591502")]
    AttMsWmiIntflags1 = 0x9068e,

    #[strum(serialize = "ATTj591503")]
    AttMsWmiIntflags2 = 0x9068f,

    #[strum(serialize = "ATTj591504")]
    AttMsWmiIntflags3 = 0x90690,

    #[strum(serialize = "ATTj591505")]
    AttMsWmiIntflags4 = 0x90691,

    #[strum(serialize = "ATTj591453")]
    AttMsWmiIntmax = 0x9065d,

    #[strum(serialize = "ATTj591454")]
    AttMsWmiIntmin = 0x9065e,

    #[strum(serialize = "ATTj591455")]
    AttMsWmiIntvalidvalues = 0x9065f,

    #[strum(serialize = "ATTq591456")]
    AttMsWmiInt8Default = 0x90660,

    #[strum(serialize = "ATTq591457")]
    AttMsWmiInt8Max = 0x90661,

    #[strum(serialize = "ATTq591458")]
    AttMsWmiInt8Min = 0x90662,

    #[strum(serialize = "ATTq591459")]
    AttMsWmiInt8Validvalues = 0x90663,

    #[strum(serialize = "ATTm591462")]
    AttMsWmiMof = 0x90666,

    #[strum(serialize = "ATTm591463")]
    AttMsWmiName = 0x90667,

    #[strum(serialize = "ATTm591464")]
    AttMsWmiNormalizedclass = 0x90668,

    #[strum(serialize = "ATTm591506")]
    AttMsWmiParm1 = 0x90692,

    #[strum(serialize = "ATTm591507")]
    AttMsWmiParm2 = 0x90693,

    #[strum(serialize = "ATTm591508")]
    AttMsWmiParm3 = 0x90694,

    #[strum(serialize = "ATTm591509")]
    AttMsWmiParm4 = 0x90695,

    #[strum(serialize = "ATTm591465")]
    AttMsWmiPropertyname = 0x90669,

    #[strum(serialize = "ATTm591466")]
    AttMsWmiQuery = 0x9066a,

    #[strum(serialize = "ATTm591467")]
    AttMsWmiQuerylanguage = 0x9066b,

    #[strum(serialize = "ATTm591510")]
    AttMsWmiScopeguid = 0x90696,

    #[strum(serialize = "ATTm591468")]
    AttMsWmiSourceorganization = 0x9066c,

    #[strum(serialize = "ATTm591460")]
    AttMsWmiStringdefault = 0x90664,

    #[strum(serialize = "ATTm591461")]
    AttMsWmiStringvalidvalues = 0x90665,

    #[strum(serialize = "ATTm591469")]
    AttMsWmiTargetclass = 0x9066d,

    #[strum(serialize = "ATTm591470")]
    AttMsWmiTargetnamespace = 0x9066e,

    #[strum(serialize = "ATTk591471")]
    AttMsWmiTargetobject = 0x9066f,

    #[strum(serialize = "ATTm591472")]
    AttMsWmiTargetpath = 0x90670,

    #[strum(serialize = "ATTm591473")]
    AttMsWmiTargettype = 0x90671,

    #[strum(serialize = "ATTf590540")]
    AttMscopeId = 0x902cc,

    #[strum(serialize = "ATTm590495")]
    AttMsiFileList = 0x9029f,

    #[strum(serialize = "ATTk590638")]
    AttMsiScript = 0x9032e,

    #[strum(serialize = "ATTm590669")]
    AttMsiScriptName = 0x9034d,

    #[strum(serialize = "ATTm589839")]
    AttMsiScriptPath = 0x9000f,

    #[strum(serialize = "ATTj590670")]
    AttMsiScriptSize = 0x9034e,

    #[strum(serialize = "ATTi590747")]
    AttMsmqAuthenticate = 0x9039b,

    #[strum(serialize = "ATTj590744")]
    AttMsmqBasePriority = 0x90398,

    #[strum(serialize = "ATTe590757")]
    AttMsmqComputerType = 0x903a5,

    #[strum(serialize = "ATTm591241")]
    AttMsmqComputerTypeEx = 0x90589,

    #[strum(serialize = "ATTj590770")]
    AttMsmqCost = 0x903b2,

    #[strum(serialize = "ATTe590764")]
    AttMsmqCspName = 0x903ac,

    #[strum(serialize = "ATTi591063")]
    AttMsmqDependentClientService = 0x904d7,

    #[strum(serialize = "ATTi591050")]
    AttMsmqDependentClientServices = 0x904ca,

    #[strum(serialize = "ATTk590772")]
    AttMsmqDigests = 0x903b4,

    #[strum(serialize = "ATTk590790")]
    AttMsmqDigestsMig = 0x903c6,

    #[strum(serialize = "ATTi591062")]
    AttMsmqDsService = 0x904d6,

    #[strum(serialize = "ATTi591052")]
    AttMsmqDsServices = 0x904cc,

    #[strum(serialize = "ATTk590760")]
    AttMsmqEncryptKey = 0x903a8,

    #[strum(serialize = "ATTi590758")]
    AttMsmqForeign = 0x903a6,

    #[strum(serialize = "ATTb590753")]
    AttMsmqInRoutingServers = 0x903a1,

    #[strum(serialize = "ATTj591132")]
    AttMsmqInterval1 = 0x9051c,

    #[strum(serialize = "ATTj591133")]
    AttMsmqInterval2 = 0x9051d,

    #[strum(serialize = "ATTi590742")]
    AttMsmqJournal = 0x90396,

    #[strum(serialize = "ATTj590745")]
    AttMsmqJournalQuota = 0x90399,

    #[strum(serialize = "ATTe590746")]
    AttMsmqLabel = 0x9039a,

    #[strum(serialize = "ATTm591239")]
    AttMsmqLabelEx = 0x90587,

    #[strum(serialize = "ATTj590765")]
    AttMsmqLongLived = 0x903ad,

    #[strum(serialize = "ATTi590776")]
    AttMsmqMigrated = 0x903b8,

    #[strum(serialize = "ATTm591538")]
    AttMsmqMulticastAddress = 0x906b2,

    #[strum(serialize = "ATTi590763")]
    AttMsmqNameStyle = 0x903ab,

    #[strum(serialize = "ATTj590788")]
    AttMsmqNt4Flags = 0x903c4,

    #[strum(serialize = "ATTj590784")]
    AttMsmqNt4Stub = 0x903c0,

    #[strum(serialize = "ATTj590759")]
    AttMsmqOsType = 0x903a7,

    #[strum(serialize = "ATTb590752")]
    AttMsmqOutRoutingServers = 0x903a0,

    #[strum(serialize = "ATTk590749")]
    AttMsmqOwnerId = 0x9039d,

    #[strum(serialize = "ATTb591049")]
    AttMsmqPrevSiteGates = 0x904c9,

    #[strum(serialize = "ATTj590748")]
    AttMsmqPrivacyLevel = 0x9039c,

    #[strum(serialize = "ATTk590775")]
    AttMsmqQmId = 0x903b7,

    #[strum(serialize = "ATTj590787")]
    AttMsmqQueueJournalQuota = 0x903c3,

    #[strum(serialize = "ATTm591067")]
    AttMsmqQueueNameExt = 0x904db,

    #[strum(serialize = "ATTj590786")]
    AttMsmqQueueQuota = 0x903c2,

    #[strum(serialize = "ATTk590741")]
    AttMsmqQueueType = 0x90395,

    #[strum(serialize = "ATTj590743")]
    AttMsmqQuota = 0x90397,

    #[strum(serialize = "ATTm591519")]
    AttMsmqRecipientFormatname = 0x9069f,

    #[strum(serialize = "ATTi591061")]
    AttMsmqRoutingService = 0x904d5,

    #[strum(serialize = "ATTi591051")]
    AttMsmqRoutingServices = 0x904cb,

    #[strum(serialize = "ATTi591537")]
    AttMsmqSecuredSource = 0x906b1,

    #[strum(serialize = "ATTj590754")]
    AttMsmqServiceType = 0x903a2,

    #[strum(serialize = "ATTj590774")]
    AttMsmqServices = 0x903b6,

    #[strum(serialize = "ATTk590771")]
    AttMsmqSignCertificates = 0x903b3,

    #[strum(serialize = "ATTk590791")]
    AttMsmqSignCertificatesMig = 0x903c7,

    #[strum(serialize = "ATTk590761")]
    AttMsmqSignKey = 0x903a9,

    #[strum(serialize = "ATTb590767")]
    AttMsmqSite1 = 0x903af,

    #[strum(serialize = "ATTb590768")]
    AttMsmqSite2 = 0x903b0,

    #[strum(serialize = "ATTi590785")]
    AttMsmqSiteForeign = 0x903c1,

    #[strum(serialize = "ATTb590769")]
    AttMsmqSiteGates = 0x903b1,

    #[strum(serialize = "ATTb591134")]
    AttMsmqSiteGatesMig = 0x9051e,

    #[strum(serialize = "ATTk590777")]
    AttMsmqSiteId = 0x903b9,

    #[strum(serialize = "ATTe590789")]
    AttMsmqSiteName = 0x903c5,

    #[strum(serialize = "ATTm591240")]
    AttMsmqSiteNameEx = 0x90588,

    #[strum(serialize = "ATTk590751")]
    AttMsmqSites = 0x9039f,

    #[strum(serialize = "ATTi590750")]
    AttMsmqTransactional = 0x9039e,

    #[strum(serialize = "ATTk591161")]
    AttMsmqUserSid = 0x90539,

    #[strum(serialize = "ATTj590766")]
    AttMsmqVersion = 0x903ae,

    #[strum(serialize = "ATTi590943")]
    AttMsnpallowdialin = 0x9045f,

    #[strum(serialize = "ATTf590947")]
    AttMsnpcalledstationid = 0x90463,

    #[strum(serialize = "ATTf590948")]
    AttMsnpcallingstationid = 0x90464,

    #[strum(serialize = "ATTf590954")]
    AttMsnpsavedcallingstationid = 0x9046a,

    #[strum(serialize = "ATTf590969")]
    AttMsradiuscallbacknumber = 0x90479,

    #[strum(serialize = "ATTj590977")]
    AttMsradiusframedipaddress = 0x90481,

    #[strum(serialize = "ATTf590982")]
    AttMsradiusframedroute = 0x90486,

    #[strum(serialize = "ATTj590995")]
    AttMsradiusservicetype = 0x90493,

    #[strum(serialize = "ATTf591013")]
    AttMsrassavedcallbacknumber = 0x904a5,

    #[strum(serialize = "ATTj591014")]
    AttMsrassavedframedipaddress = 0x904a6,

    #[strum(serialize = "ATTf591015")]
    AttMsrassavedframedroute = 0x904a7,

    #[strum(serialize = "ATTc131096")]
    AttMustContain = 0x20018,

    #[strum(serialize = "ATTj590577")]
    AttNameServiceFlags = 0x902f1,

    #[strum(serialize = "ATTb131088")]
    AttNcName = 0x20010,

    #[strum(serialize = "ATTm589911")]
    AttNetbiosName = 0x90057,

    #[strum(serialize = "ATTi590673")]
    AttNetbootAllowNewClients = 0x90351,

    #[strum(serialize = "ATTi590678")]
    AttNetbootAnswerOnlyValidClients = 0x90356,

    #[strum(serialize = "ATTi590677")]
    AttNetbootAnswerRequests = 0x90355,

    #[strum(serialize = "ATTj590676")]
    AttNetbootCurrentClientCount = 0x90354,

    #[strum(serialize = "ATTk590183")]
    AttNetbootGuid = 0x90167,

    #[strum(serialize = "ATTm590182")]
    AttNetbootInitialization = 0x90166,

    #[strum(serialize = "ATTm590681")]
    AttNetbootIntellimirrorOses = 0x90359,

    #[strum(serialize = "ATTi590674")]
    AttNetbootLimitClients = 0x90352,

    #[strum(serialize = "ATTm590683")]
    AttNetbootLocallyInstalledOses = 0x9035b,

    #[strum(serialize = "ATTm590185")]
    AttNetbootMachineFilePath = 0x90169,

    #[strum(serialize = "ATTj590675")]
    AttNetbootMaxClients = 0x90353,

    #[strum(serialize = "ATTm591065")]
    AttNetbootMirrorDataFile = 0x904d9,

    #[strum(serialize = "ATTm590679")]
    AttNetbootNewMachineNamingPolicy = 0x90357,

    #[strum(serialize = "ATTb590680")]
    AttNetbootNewMachineOu = 0x90358,

    #[strum(serialize = "ATTb590688")]
    AttNetbootScpBl = 0x90360,

    #[strum(serialize = "ATTb590684")]
    AttNetbootServer = 0x9035c,

    #[strum(serialize = "ATTm591064")]
    AttNetbootSifFile = 0x904d8,

    #[strum(serialize = "ATTm590682")]
    AttNetbootTools = 0x9035a,

    #[strum(serialize = "ATTe131531")]
    AttNetworkAddress = 0x201cb,

    #[strum(serialize = "ATTb590038")]
    AttNextLevelStore = 0x900d6,

    #[strum(serialize = "ATTj589912")]
    AttNextRid = 0x90058,

    #[strum(serialize = "ATTb590354")]
    AttNonSecurityMember = 0x90212,

    #[strum(serialize = "ATTb590355")]
    AttNonSecurityMemberBl = 0x90213,

    #[strum(serialize = "ATTb590127")]
    AttNotificationList = 0x9012f,

    #[strum(serialize = "ATTk589913")]
    AttNtGroupMembers = 0x90059,

    #[strum(serialize = "ATTj590181")]
    AttNtMixedDomain = 0x90165,

    #[strum(serialize = "ATTk589918")]
    AttNtPwdHistory = 0x9005e,

    #[strum(serialize = "ATTp131353")]
    AttNtSecurityDescriptor = 0x20119,

    #[strum(serialize = "ATTb49")]
    AttObjDistName = 0x31,

    #[strum(serialize = "ATTb590606")]
    AttObjectCategory = 0x9030e,

    #[strum(serialize = "ATTc0")]
    AttObjectClass = 0x0,

    #[strum(serialize = "ATTj131442")]
    AttObjectClassCategory = 0x20172,

    #[strum(serialize = "ATTm1572870")]
    AttObjectClasses = 0x180006,

    #[strum(serialize = "ATTj590330")]
    AttObjectCount = 0x901fa,

    #[strum(serialize = "ATTk589826")]
    AttObjectGuid = 0x90002,

    #[strum(serialize = "ATTr589970")]
    AttObjectSid = 0x90092,

    #[strum(serialize = "ATTj131148")]
    AttObjectVersion = 0x2004c,

    #[strum(serialize = "ATTm589975")]
    AttOemInformation = 0x90097,

    #[strum(serialize = "ATTk131290")]
    AttOmObjectClass = 0x200da,

    #[strum(serialize = "ATTj131303")]
    AttOmSyntax = 0x200e7,

    #[strum(serialize = "ATTk590329")]
    AttOmtGuid = 0x901f9,

    #[strum(serialize = "ATTk590157")]
    AttOmtIndxGuid = 0x9014d,

    #[strum(serialize = "ATTm590187")]
    AttOperatingSystem = 0x9016b,

    #[strum(serialize = "ATTm590239")]
    AttOperatingSystemHotfix = 0x9019f,

    #[strum(serialize = "ATTm590189")]
    AttOperatingSystemServicePack = 0x9016d,

    #[strum(serialize = "ATTm590188")]
    AttOperatingSystemVersion = 0x9016c,

    #[strum(serialize = "ATTj589968")]
    AttOperatorCount = 0x90090,

    #[strum(serialize = "ATTm590536")]
    AttOptionDescription = 0x902c8,

    #[strum(serialize = "ATTj590131")]
    AttOptions = 0x90133,

    #[strum(serialize = "ATTf590537")]
    AttOptionsLocation = 0x902c9,

    #[strum(serialize = "ATTm10")]
    AttOrganizationName = 0xa,

    #[strum(serialize = "ATTm11")]
    AttOrganizationalUnitName = 0xb,

    #[strum(serialize = "ATTk131517")]
    AttOriginalDisplayTable = 0x201bd,

    #[strum(serialize = "ATTk131286")]
    AttOriginalDisplayTableMsdos = 0x200d6,

    #[strum(serialize = "ATTm589915")]
    AttOtherLoginWorkstations = 0x9005b,

    #[strum(serialize = "ATTm590475")]
    AttOtherMailbox = 0x9028b,

    #[strum(serialize = "ATTm1441826")]
    AttOtherName = 0x160022,

    #[strum(serialize = "ATTh591183")]
    AttOtherWellKnownObjects = 0x9054f,

    #[strum(serialize = "ATTb32")]
    AttOwner = 0x20,

    #[strum(serialize = "ATTj590151")]
    AttPackageFlags = 0x90147,

    #[strum(serialize = "ATTm590150")]
    AttPackageName = 0x90146,

    #[strum(serialize = "ATTj590148")]
    AttPackageType = 0x90144,

    #[strum(serialize = "ATTb590381")]
    AttParentCa = 0x9022d,

    #[strum(serialize = "ATTk590509")]
    AttParentCaCertificateChain = 0x902ad,

    #[strum(serialize = "ATTk591048")]
    AttParentGuid = 0x904c8,

    #[strum(serialize = "ATTk590487")]
    AttPartialAttributeDeletionList = 0x90297,

    #[strum(serialize = "ATTk590464")]
    AttPartialAttributeSet = 0x90280,

    #[strum(serialize = "ATTq590690")]
    AttPekKeyChangeInterval = 0x90362,

    #[strum(serialize = "ATTk590689")]
    AttPekList = 0x90361,

    #[strum(serialize = "ATTk590517")]
    AttPendingCaCertificates = 0x902b5,

    #[strum(serialize = "ATTb590519")]
    AttPendingParentCa = 0x902b7,

    #[strum(serialize = "ATTk131397")]
    AttPerMsgDialogDisplayTable = 0x20145,

    #[strum(serialize = "ATTk131398")]
    AttPerRecipDialogDisplayTable = 0x20146,

    #[strum(serialize = "ATTm131687")]
    AttPersonalTitle = 0x20267,

    #[strum(serialize = "ATTm590470")]
    AttPhoneFaxOther = 0x90286,

    #[strum(serialize = "ATTm131349")]
    AttPhoneHomeOther = 0x20115,

    #[strum(serialize = "ATTm1376276")]
    AttPhoneHomePrimary = 0x150014,

    #[strum(serialize = "ATTm590546")]
    AttPhoneIpOther = 0x902d2,

    #[strum(serialize = "ATTm590545")]
    AttPhoneIpPrimary = 0x902d1,

    #[strum(serialize = "ATTm590473")]
    AttPhoneIsdnPrimary = 0x90289,

    #[strum(serialize = "ATTm590471")]
    AttPhoneMobileOther = 0x90287,

    #[strum(serialize = "ATTm1376297")]
    AttPhoneMobilePrimary = 0x150029,

    #[strum(serialize = "ATTm131090")]
    AttPhoneOfficeOther = 0x20012,

    #[strum(serialize = "ATTm131190")]
    AttPhonePagerOther = 0x20076,

    #[strum(serialize = "ATTm1376298")]
    AttPhonePagerPrimary = 0x15002a,

    #[strum(serialize = "ATTk1376263")]
    AttPhoto = 0x150007,

    #[strum(serialize = "ATTm19")]
    AttPhysicalDeliveryOfficeName = 0x13,

    #[strum(serialize = "ATTb590338")]
    AttPhysicalLocationObject = 0x90202,

    #[strum(serialize = "ATTk1441827")]
    AttPicture = 0x160023,

    #[strum(serialize = "ATTm591154")]
    AttPkiCriticalExtensions = 0x90532,

    #[strum(serialize = "ATTm591158")]
    AttPkiDefaultCsps = 0x90536,

    #[strum(serialize = "ATTj591151")]
    AttPkiDefaultKeySpec = 0x9052f,

    #[strum(serialize = "ATTp591159")]
    AttPkiEnrollmentAccess = 0x90537,

    #[strum(serialize = "ATTk591155")]
    AttPkiExpirationPeriod = 0x90533,

    #[strum(serialize = "ATTm591157")]
    AttPkiExtendedKeyUsage = 0x90535,

    #[strum(serialize = "ATTk591152")]
    AttPkiKeyUsage = 0x90530,

    #[strum(serialize = "ATTj591153")]
    AttPkiMaxIssuingDepth = 0x90531,

    #[strum(serialize = "ATTk591156")]
    AttPkiOverlapPeriod = 0x90534,

    #[strum(serialize = "ATTk590030")]
    AttPkt = 0x900ce,

    #[strum(serialize = "ATTk590029")]
    AttPktGuid = 0x900cd,

    #[strum(serialize = "ATTj590457")]
    AttPolicyReplicationFlags = 0x90279,

    #[strum(serialize = "ATTm590052")]
    AttPortName = 0x900e4,

    #[strum(serialize = "ATTc131080")]
    AttPossSuperiors = 0x20008,

    #[strum(serialize = "ATTc590739")]
    AttPossibleInferiors = 0x90393,

    #[strum(serialize = "ATTm18")]
    AttPostOfficeBox = 0x12,

    #[strum(serialize = "ATTm16")]
    AttPostalAddress = 0x10,

    #[strum(serialize = "ATTm17")]
    AttPostalCode = 0x11,

    #[strum(serialize = "ATTj28")]
    AttPreferredDeliveryMethod = 0x1c,

    #[strum(serialize = "ATTm1441831")]
    AttPreferredlanguage = 0x160027,

    #[strum(serialize = "ATTb589921")]
    AttPreferredOu = 0x90061,

    #[strum(serialize = "ATTk590362")]
    AttPrefixMap = 0x9021a,

    #[strum(serialize = "ATTn29")]
    AttPresentationAddress = 0x1d,

    #[strum(serialize = "ATTk590516")]
    AttPreviousCaCertificates = 0x902b4,

    #[strum(serialize = "ATTb590518")]
    AttPreviousParentCa = 0x902b6,

    #[strum(serialize = "ATTj589922")]
    AttPrimaryGroupId = 0x90062,

    #[strum(serialize = "ATTj591236")]
    AttPrimaryGroupToken = 0x90584,

    #[strum(serialize = "ATTj590071")]
    AttPrintAttributes = 0x900f7,

    #[strum(serialize = "ATTm590061")]
    AttPrintBinNames = 0x900ed,

    #[strum(serialize = "ATTi590066")]
    AttPrintCollate = 0x900f2,

    #[strum(serialize = "ATTi590067")]
    AttPrintColor = 0x900f3,

    #[strum(serialize = "ATTi591135")]
    AttPrintDuplexSupported = 0x9051f,

    #[strum(serialize = "ATTj590058")]
    AttPrintEndTime = 0x900ea,

    #[strum(serialize = "ATTm590059")]
    AttPrintFormName = 0x900eb,

    #[strum(serialize = "ATTi590099")]
    AttPrintKeepPrintedJobs = 0x90113,

    #[strum(serialize = "ATTm590070")]
    AttPrintLanguage = 0x900f6,

    #[strum(serialize = "ATTm590112")]
    AttPrintMacAddress = 0x90120,

    #[strum(serialize = "ATTj590065")]
    AttPrintMaxCopies = 0x900f1,

    #[strum(serialize = "ATTj590062")]
    AttPrintMaxResolutionSupported = 0x900ee,

    #[strum(serialize = "ATTj590101")]
    AttPrintMaxXExtent = 0x90115,

    #[strum(serialize = "ATTj590102")]
    AttPrintMaxYExtent = 0x90116,

    #[strum(serialize = "ATTm590113")]
    AttPrintMediaReady = 0x90121,

    #[strum(serialize = "ATTm590123")]
    AttPrintMediaSupported = 0x9012b,

    #[strum(serialize = "ATTj590106")]
    AttPrintMemory = 0x9011a,

    #[strum(serialize = "ATTj590103")]
    AttPrintMinXExtent = 0x90117,

    #[strum(serialize = "ATTj590104")]
    AttPrintMinYExtent = 0x90118,

    #[strum(serialize = "ATTm590111")]
    AttPrintNetworkAddress = 0x9011f,

    #[strum(serialize = "ATTm590096")]
    AttPrintNotify = 0x90110,

    #[strum(serialize = "ATTj590114")]
    AttPrintNumberUp = 0x90122,

    #[strum(serialize = "ATTm590064")]
    AttPrintOrientationsSupported = 0x900f0,

    #[strum(serialize = "ATTm590095")]
    AttPrintOwner = 0x9010f,

    #[strum(serialize = "ATTj590455")]
    AttPrintPagesPerMinute = 0x90277,

    #[strum(serialize = "ATTj590109")]
    AttPrintRate = 0x9011d,

    #[strum(serialize = "ATTm590110")]
    AttPrintRateUnit = 0x9011e,

    #[strum(serialize = "ATTm590054")]
    AttPrintSeparatorFile = 0x900e6,

    #[strum(serialize = "ATTm590094")]
    AttPrintShareName = 0x9010e,

    #[strum(serialize = "ATTm590098")]
    AttPrintSpooling = 0x90112,

    #[strum(serialize = "ATTi590105")]
    AttPrintStaplingSupported = 0x90119,

    #[strum(serialize = "ATTj590057")]
    AttPrintStartTime = 0x900e9,

    #[strum(serialize = "ATTm590097")]
    AttPrintStatus = 0x90111,

    #[strum(serialize = "ATTm590124")]
    AttPrinterName = 0x9012c,

    #[strum(serialize = "ATTq589923")]
    AttPriorSetTime = 0x90063,

    #[strum(serialize = "ATTk589924")]
    AttPriorValue = 0x90064,

    #[strum(serialize = "ATTj590055")]
    AttPriority = 0x900e7,

    #[strum(serialize = "ATTk589925")]
    AttPrivateKey = 0x90065,

    #[strum(serialize = "ATTj590460")]
    AttPrivilegeAttributes = 0x9027c,

    #[strum(serialize = "ATTm590458")]
    AttPrivilegeDisplayName = 0x9027a,

    #[strum(serialize = "ATTb590461")]
    AttPrivilegeHolder = 0x9027d,

    #[strum(serialize = "ATTq590459")]
    AttPrivilegeValue = 0x9027b,

    #[strum(serialize = "ATTk590642")]
    AttProductCode = 0x90332,

    #[strum(serialize = "ATTm589963")]
    AttProfilePath = 0x9008b,

    #[strum(serialize = "ATTh591073")]
    AttProxiedObjectName = 0x904e1,

    #[strum(serialize = "ATTm131282")]
    AttProxyAddresses = 0x200d2,

    #[strum(serialize = "ATTi131595")]
    AttProxyGenerationEnabled = 0x2020b,

    #[strum(serialize = "ATTq589927")]
    AttProxyLifetime = 0x90067,

    #[strum(serialize = "ATTk590244")]
    AttPublicKeyPolicy = 0x901a4,

    #[strum(serialize = "ATTm590710")]
    AttPurportedSearch = 0x90376,

    #[strum(serialize = "ATTj589919")]
    AttPwdHistoryLength = 0x9005f,

    #[strum(serialize = "ATTq589920")]
    AttPwdLastSet = 0x90060,

    #[strum(serialize = "ATTj589917")]
    AttPwdProperties = 0x9005d,

    #[strum(serialize = "ATTj590282")]
    AttQualityOfService = 0x901ca,

    #[strum(serialize = "ATTm591179")]
    AttQueryFilter = 0x9054b,

    #[strum(serialize = "ATTb590432")]
    AttQueryPolicyBl = 0x90260,

    #[strum(serialize = "ATTb590431")]
    AttQueryPolicyObject = 0x9025f,

    #[strum(serialize = "ATTm590504")]
    AttQuerypoint = 0x902a8,

    #[strum(serialize = "ATTj131106")]
    AttRangeLower = 0x20022,

    #[strum(serialize = "ATTj131107")]
    AttRangeUpper = 0x20023,

    #[strum(serialize = "ATTm589825")]
    AttRdn = 0x90001,

    #[strum(serialize = "ATTc131098")]
    AttRdnAttId = 0x2001a,

    #[strum(serialize = "ATTk26")]
    AttRegisteredAddress = 0x1a,

    #[strum(serialize = "ATTm589929")]
    AttRemoteServerName = 0x90069,

    #[strum(serialize = "ATTm589931")]
    AttRemoteSource = 0x9006b,

    #[strum(serialize = "ATTj589932")]
    AttRemoteSourceType = 0x9006c,

    #[strum(serialize = "ATTm590633")]
    AttRemoteStorageGuid = 0x90329,

    #[strum(serialize = "ATTk589827")]
    AttReplPropertyMetaData = 0x90003,

    #[strum(serialize = "ATTj590501")]
    AttReplTopologyStayOfExecution = 0x902a5,

    #[strum(serialize = "ATTk589828")]
    AttReplUptodateVector = 0x90004,

    #[strum(serialize = "ATTm589933")]
    AttReplicaSource = 0x9006d,

    #[strum(serialize = "ATTb131508")]
    AttReports = 0x201b4,

    #[strum(serialize = "ATTj591160")]
    AttReplInterval = 0x90538,

    #[strum(serialize = "ATTk131163")]
    AttRepsFrom = 0x2005b,

    #[strum(serialize = "ATTk131155")]
    AttRepsTo = 0x20053,

    #[strum(serialize = "ATTk590145")]
    AttRequiredCategories = 0x90141,

    #[strum(serialize = "ATTk590497")]
    AttRetiredReplDsaSignatures = 0x902a1,

    #[strum(serialize = "ATTr591125")]
    AttTokenGroups = 0x90515,

    #[strum(serialize = "ATTr591242")]
    AttTokenGroupsGlobalAndUniversal = 0x9058a,

    #[strum(serialize = "ATTr591127")]
    AttTokenGroupsNoGcAcceptable = 0x90517,

    #[strum(serialize = "ATTj589969")]
    AttRevision = 0x90091,

    #[strum(serialize = "ATTj589977")]
    AttRid = 0x90099,

    #[strum(serialize = "ATTq590195")]
    AttRidAllocationPool = 0x90173,

    #[strum(serialize = "ATTq590194")]
    AttRidAvailablePool = 0x90172,

    #[strum(serialize = "ATTb590192")]
    AttRidManagerReference = 0x90170,

    #[strum(serialize = "ATTj590198")]
    AttRidNextRid = 0x90176,

    #[strum(serialize = "ATTq590196")]
    AttRidPreviousAllocationPool = 0x90174,

    #[strum(serialize = "ATTb590493")]
    AttRidSetReferences = 0x9029d,

    #[strum(serialize = "ATTq590197")]
    AttRidUsedPool = 0x90175,

    #[strum(serialize = "ATTm590164")]
    AttRightsGuid = 0x90154,

    #[strum(serialize = "ATTb33")]
    AttRoleOccupant = 0x21,

    #[strum(serialize = "ATTm1376262")]
    AttRoomnumber = 0x150006,

    #[strum(serialize = "ATTb590498")]
    AttRootTrust = 0x902a2,

    #[strum(serialize = "ATTm590190")]
    AttRpcNsAnnotation = 0x9016e,

    #[strum(serialize = "ATTm589937")]
    AttRpcNsBindings = 0x90071,

    #[strum(serialize = "ATTm590191")]
    AttRpcNsCodeset = 0x9016f,

    #[strum(serialize = "ATTj590578")]
    AttRpcNsEntryFlags = 0x902f2,

    #[strum(serialize = "ATTm589938")]
    AttRpcNsGroup = 0x90072,

    #[strum(serialize = "ATTm589939")]
    AttRpcNsInterfaceId = 0x90073,

    #[strum(serialize = "ATTm590136")]
    AttRpcNsObjectId = 0x90138,

    #[strum(serialize = "ATTj589941")]
    AttRpcNsPriority = 0x90075,

    #[strum(serialize = "ATTm589942")]
    AttRpcNsProfileEntry = 0x90076,

    #[strum(serialize = "ATTm590138")]
    AttRpcNsTransferSyntax = 0x9013a,

    #[strum(serialize = "ATTm590045")]
    AttSamAccountName = 0x900dd,

    #[strum(serialize = "ATTj590126")]
    AttSamAccountType = 0x9012e,

    #[strum(serialize = "ATTk590035")]
    AttSchedule = 0x900d3,

    #[strum(serialize = "ATTj589944")]
    AttSchemaFlagsEx = 0x90078,

    #[strum(serialize = "ATTk589972")]
    AttSchemaIdGuid = 0x90094,

    #[strum(serialize = "ATTk591182")]
    AttSchemaInfo = 0x9054e,

    #[strum(serialize = "ATTl590305")]
    AttSchemaUpdate = 0x901e1,

    #[strum(serialize = "ATTj131543")]
    AttSchemaVersion = 0x201d7,

    #[strum(serialize = "ATTj591178")]
    AttScopeFlags = 0x9054a,

    #[strum(serialize = "ATTm589886")]
    AttScriptPath = 0x9003e,

    #[strum(serialize = "ATTj591128")]
    AttSdRightsEffective = 0x90518,

    #[strum(serialize = "ATTj131406")]
    AttSearchFlags = 0x2014e,

    #[strum(serialize = "ATTk14")]
    AttSearchGuide = 0xe,

    #[strum(serialize = "ATTb1376277")]
    AttSecretary = 0x150015,

    #[strum(serialize = "ATTr589945")]
    AttSecurityIdentifier = 0x90079,

    #[strum(serialize = "ATTb34")]
    AttSeeAlso = 0x22,

    #[strum(serialize = "ATTj590328")]
    AttSeqNotification = 0x901f8,

    #[strum(serialize = "ATTf5")]
    AttSerialNumber = 0x5,

    #[strum(serialize = "ATTm590047")]
    AttServerName = 0x900df,

    #[strum(serialize = "ATTb590339")]
    AttServerReference = 0x90203,

    #[strum(serialize = "ATTb590340")]
    AttServerReferenceBl = 0x90204,

    #[strum(serialize = "ATTj589981")]
    AttServerRole = 0x9009d,

    #[strum(serialize = "ATTj589978")]
    AttServerState = 0x9009a,

    #[strum(serialize = "ATTm590334")]
    AttServiceBindingInformation = 0x901fe,

    #[strum(serialize = "ATTk589946")]
    AttServiceClassId = 0x9007a,

    #[strum(serialize = "ATTk589947")]
    AttServiceClassInfo = 0x9007b,

    #[strum(serialize = "ATTm590333")]
    AttServiceClassName = 0x901fd,

    #[strum(serialize = "ATTm590481")]
    AttServiceDnsName = 0x90291,

    #[strum(serialize = "ATTm590483")]
    AttServiceDnsNameType = 0x90293,

    #[strum(serialize = "ATTk590023")]
    AttServiceInstanceVersion = 0x900c7,

    #[strum(serialize = "ATTm590595")]
    AttServicePrincipalName = 0x90303,

    #[strum(serialize = "ATTm590149")]
    AttSetupCommand = 0x90145,

    #[strum(serialize = "ATTm590439")]
    AttShellContextMenu = 0x90267,

    #[strum(serialize = "ATTm590387")]
    AttShellPropertyPages = 0x90233,

    #[strum(serialize = "ATTm591033")]
    AttShortServerName = 0x904b9,

    #[strum(serialize = "ATTb590468")]
    AttShowInAddressBook = 0x90284,

    #[strum(serialize = "ATTi131241")]
    AttShowInAdvancedViewOnly = 0x200a9,

    #[strum(serialize = "ATTr590433")]
    AttSidHistory = 0x90261,

    #[strum(serialize = "ATTm590648")]
    AttSignatureAlgorithms = 0x90338,

    #[strum(serialize = "ATTk590186")]
    AttSiteGuid = 0x9016a,

    #[strum(serialize = "ATTb590646")]
    AttSiteLinkList = 0x90336,

    #[strum(serialize = "ATTb590645")]
    AttSiteList = 0x90335,

    #[strum(serialize = "ATTb590336")]
    AttSiteObject = 0x90200,

    #[strum(serialize = "ATTb590337")]
    AttSiteObjectBl = 0x90201,

    #[strum(serialize = "ATTb590318")]
    AttSiteServer = 0x901ee,

    #[strum(serialize = "ATTm590610")]
    AttSmtpMailAddress = 0x90312,

    #[strum(serialize = "ATTm591171")]
    AttSpnMappings = 0x90543,

    #[strum(serialize = "ATTm8")]
    AttStateOrProvinceName = 0x8,

    #[strum(serialize = "ATTm9")]
    AttStreetAddress = 0x9,

    #[strum(serialize = "ATTc1572873")]
    AttStructuralObjectClass = 0x180009,

    #[strum(serialize = "ATTc131093")]
    AttSubClassOf = 0x20015,

    #[strum(serialize = "ATTb131079")]
    AttSubRefs = 0x20007,

    #[strum(serialize = "ATTb1638410")]
    AttSubschemasubentry = 0x19000a,

    #[strum(serialize = "ATTm590535")]
    AttSuperScopeDescription = 0x902c7,

    #[strum(serialize = "ATTf590534")]
    AttSuperScopes = 0x902c6,

    #[strum(serialize = "ATTm590356")]
    AttSuperiorDnsRoot = 0x90214,

    #[strum(serialize = "ATTk589949")]
    AttSupplementalCredentials = 0x9007d,

    #[strum(serialize = "ATTk30")]
    AttSupportedApplicationContext = 0x1e,

    #[strum(serialize = "ATTm4")]
    AttSurname = 0x4,

    #[strum(serialize = "ATTj590490")]
    AttSyncAttributes = 0x9029a,

    #[strum(serialize = "ATTb590489")]
    AttSyncMembership = 0x90299,

    #[strum(serialize = "ATTb590488")]
    AttSyncWithObject = 0x90298,

    #[strum(serialize = "ATTr590491")]
    AttSyncWithSid = 0x9029b,

    #[strum(serialize = "ATTc590022")]
    AttSystemAuxiliaryClass = 0x900c6,

    #[strum(serialize = "ATTj590199")]
    AttSystemFlags = 0x90177,

    #[strum(serialize = "ATTc590020")]
    AttSystemMayContain = 0x900c4,

    #[strum(serialize = "ATTc590021")]
    AttSystemMustContain = 0x900c5,

    #[strum(serialize = "ATTi589994")]
    AttSystemOnly = 0x900aa,

    #[strum(serialize = "ATTc590019")]
    AttSystemPossSuperiors = 0x900c3,

    #[strum(serialize = "ATTm20")]
    AttTelephoneNumber = 0x14,

    #[strum(serialize = "ATTk22")]
    AttTeletexTerminalIdentifier = 0x16,

    #[strum(serialize = "ATTk21")]
    AttTelexNumber = 0x15,

    #[strum(serialize = "ATTm590472")]
    AttTelexPrimary = 0x90288,

    #[strum(serialize = "ATTb591170")]
    AttTemplateRoots = 0x90542,

    #[strum(serialize = "ATTk590709")]
    AttTerminalServer = 0x90375,

    #[strum(serialize = "ATTm131203")]
    AttTextCountry = 0x20083,

    #[strum(serialize = "ATTm1376258")]
    AttTextEncodedOrAddress = 0x150002,

    #[strum(serialize = "ATTq590327")]
    AttTimeRefresh = 0x901f7,

    #[strum(serialize = "ATTq590326")]
    AttTimeVolChange = 0x901f6,

    #[strum(serialize = "ATTm12")]
    AttTitle = 0xc,

    #[strum(serialize = "ATTj131126")]
    AttTombstoneLifetime = 0x20036,

    #[strum(serialize = "ATTc590719")]
    AttTransportAddressAttribute = 0x9037f,

    #[strum(serialize = "ATTm590613")]
    AttTransportDllName = 0x90315,

    #[strum(serialize = "ATTb590615")]
    AttTransportType = 0x90317,

    #[strum(serialize = "ATTi590630")]
    AttTreatAsLeaf = 0x90326,

    #[strum(serialize = "ATTm590484")]
    AttTreeName = 0x90294,

    #[strum(serialize = "ATTj590294")]
    AttTrustAttributes = 0x901d6,

    #[strum(serialize = "ATTk589953")]
    AttTrustAuthIncoming = 0x90081,

    #[strum(serialize = "ATTk589959")]
    AttTrustAuthOutgoing = 0x90087,

    #[strum(serialize = "ATTj589956")]
    AttTrustDirection = 0x90084,

    #[strum(serialize = "ATTb590295")]
    AttTrustParent = 0x901d7,

    #[strum(serialize = "ATTm589957")]
    AttTrustPartner = 0x90085,

    #[strum(serialize = "ATTj589958")]
    AttTrustPosixOffset = 0x90086,

    #[strum(serialize = "ATTj589960")]
    AttTrustType = 0x90088,

    #[strum(serialize = "ATTj589979")]
    AttUasCompat = 0x9009b,

    #[strum(serialize = "ATTm1376257")]
    AttUid = 0x150001,

    #[strum(serialize = "ATTm589961")]
    AttUncName = 0x90089,

    #[strum(serialize = "ATTk589914")]
    AttUnicodePwd = 0x9005a,

    #[strum(serialize = "ATTb50")]
    AttUniquemember = 0x32,

    #[strum(serialize = "ATTk590637")]
    AttUpgradeProductCode = 0x9032d,

    #[strum(serialize = "ATTm590714")]
    AttUpnSuffixes = 0x9037a,

    #[strum(serialize = "ATTj589832")]
    AttUserAccountControl = 0x90008,

    #[strum(serialize = "ATTk590469")]
    AttUserCert = 0x90285,

    #[strum(serialize = "ATTm589980")]
    AttUserComment = 0x9009c,

    #[strum(serialize = "ATTm589962")]
    AttUserParameters = 0x9008a,

    #[strum(serialize = "ATTk35")]
    AttUserPassword = 0x23,

    #[strum(serialize = "ATTk1442008")]
    AttUserpkcs12 = 0x1600d8,

    #[strum(serialize = "ATTm590480")]
    AttUserPrincipalName = 0x90290,

    #[strum(serialize = "ATTm590575")]
    AttUserSharedFolder = 0x902ef,

    #[strum(serialize = "ATTm590576")]
    AttUserSharedFolderOther = 0x902f0,

    #[strum(serialize = "ATTk1310860")]
    AttUserSmimeCertificate = 0x14008c,

    #[strum(serialize = "ATTm589910")]
    AttUserWorkstations = 0x90056,

    #[strum(serialize = "ATTq131192")]
    AttUsnChanged = 0x20078,

    #[strum(serialize = "ATTq131091")]
    AttUsnCreated = 0x20013,

    #[strum(serialize = "ATTq131339")]
    AttUsnDsaLastObjRemoved = 0x2010b,

    #[strum(serialize = "ATTj131541")]
    AttUsnIntersite = 0x201d5,

    #[strum(serialize = "ATTq131193")]
    AttUsnLastObjRem = 0x20079,

    #[strum(serialize = "ATTq590720")]
    AttUsnSource = 0x90380,

    #[strum(serialize = "ATTj591180")]
    AttValidAccesses = 0x9054c,

    #[strum(serialize = "ATTm590079")]
    AttVendor = 0x900ff,

    #[strum(serialize = "ATTj589965")]
    AttVersionNumber = 0x9008d,

    #[strum(serialize = "ATTj590152")]
    AttVersionNumberHi = 0x90148,

    #[strum(serialize = "ATTj590153")]
    AttVersionNumberLo = 0x90149,

    #[strum(serialize = "ATTk590160")]
    AttVolTableGuid = 0x90150,

    #[strum(serialize = "ATTk590158")]
    AttVolTableIdxGuid = 0x9014e,

    #[strum(serialize = "ATTj590331")]
    AttVolumeCount = 0x901fb,

    #[strum(serialize = "ATTm590125")]
    AttWbemPath = 0x9012d,

    #[strum(serialize = "ATTh590442")]
    AttWellKnownObjects = 0x9026a,

    #[strum(serialize = "ATTl131075")]
    AttWhenChanged = 0x20003,

    #[strum(serialize = "ATTl131074")]
    AttWhenCreated = 0x20002,

    #[strum(serialize = "ATTk589966")]
    AttWinsockAddresses = 0x9008e,

    #[strum(serialize = "ATTm131536")]
    AttWwwHomePage = 0x201d0,

    #[strum(serialize = "ATTm590573")]
    AttWwwPageOther = 0x902ed,

    #[strum(serialize = "ATTg24")]
    AttX121Address = 0x18,

    #[strum(serialize = "ATTk45")]
    AttX500Uniqueidentifier = 0x2d,

    #[strum(serialize = "ATTk36")]
    AttX509Cert = 0x24,

    #[strum(serialize = "DNT_col")]
    DsRecordId = 0xffffff01,

    #[strum(serialize = "PDNT_col")]
    DsParentRecordId = 0xffffff02,

    #[strum(serialize = "time_col")]
    DsRecordTime = 0xffffff03,

    #[strum(serialize = "Ancestors_col")]
    DsAncestors = 0xffffff04,
}

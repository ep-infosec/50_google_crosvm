// This code was autogenerated with `dbus-codegen-rust -s -m None`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
#[allow(unused_imports)]
use dbus::arg;
use dbus::blocking;

pub trait OrgChromiumUserDataAuthInterface {
    fn is_mounted(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn unmount(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn mount(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn remove(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn list_keys(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_key_data(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn check_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn add_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn remove_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn mass_remove_keys(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn migrate_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn start_fingerprint_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn end_fingerprint_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_web_authn_secret(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_web_authn_secret_hash(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_hibernate_secret(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn start_migrate_to_dircrypto(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn needs_dircrypto_migration(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_supported_key_policies(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_account_disk_usage(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn start_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn add_credentials(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn update_credential(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn authenticate_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn invalidate_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn extend_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_auth_session_status(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn create_persistent_user(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn authenticate_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn prepare_guest_vault(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn prepare_ephemeral_vault(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn prepare_persistent_vault(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn prepare_vault_for_migration(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn add_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn update_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn remove_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn list_auth_factors(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn prepare_async_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_recovery_request(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn reset_application_container(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
}

#[derive(Debug)]
pub struct OrgChromiumUserDataAuthInterfaceDircryptoMigrationProgress {
    pub status: Vec<u8>,
}

impl arg::AppendAll for OrgChromiumUserDataAuthInterfaceDircryptoMigrationProgress {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.status, i);
    }
}

impl arg::ReadAll for OrgChromiumUserDataAuthInterfaceDircryptoMigrationProgress {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgChromiumUserDataAuthInterfaceDircryptoMigrationProgress {
            status: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgChromiumUserDataAuthInterfaceDircryptoMigrationProgress {
    const NAME: &'static str = "DircryptoMigrationProgress";
    const INTERFACE: &'static str = "org.chromium.UserDataAuthInterface";
}

#[derive(Debug)]
pub struct OrgChromiumUserDataAuthInterfaceLowDiskSpace {
    pub status: Vec<u8>,
}

impl arg::AppendAll for OrgChromiumUserDataAuthInterfaceLowDiskSpace {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.status, i);
    }
}

impl arg::ReadAll for OrgChromiumUserDataAuthInterfaceLowDiskSpace {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgChromiumUserDataAuthInterfaceLowDiskSpace {
            status: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgChromiumUserDataAuthInterfaceLowDiskSpace {
    const NAME: &'static str = "LowDiskSpace";
    const INTERFACE: &'static str = "org.chromium.UserDataAuthInterface";
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgChromiumUserDataAuthInterface for blocking::Proxy<'a, C> {

    fn is_mounted(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "IsMounted", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn unmount(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "Unmount", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn mount(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "Mount", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn remove(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "Remove", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn list_keys(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "ListKeys", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_key_data(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetKeyData", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn check_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "CheckKey", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn add_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "AddKey", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn remove_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "RemoveKey", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn mass_remove_keys(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "MassRemoveKeys", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn migrate_key(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "MigrateKey", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn start_fingerprint_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "StartFingerprintAuthSession", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn end_fingerprint_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "EndFingerprintAuthSession", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_web_authn_secret(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetWebAuthnSecret", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_web_authn_secret_hash(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetWebAuthnSecretHash", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_hibernate_secret(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetHibernateSecret", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn start_migrate_to_dircrypto(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "StartMigrateToDircrypto", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn needs_dircrypto_migration(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "NeedsDircryptoMigration", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_supported_key_policies(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetSupportedKeyPolicies", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_account_disk_usage(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetAccountDiskUsage", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn start_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "StartAuthSession", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn add_credentials(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "AddCredentials", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn update_credential(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "UpdateCredential", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn authenticate_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "AuthenticateAuthSession", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn invalidate_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "InvalidateAuthSession", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn extend_auth_session(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "ExtendAuthSession", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_auth_session_status(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetAuthSessionStatus", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn create_persistent_user(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "CreatePersistentUser", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn authenticate_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "AuthenticateAuthFactor", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn prepare_guest_vault(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "PrepareGuestVault", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn prepare_ephemeral_vault(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "PrepareEphemeralVault", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn prepare_persistent_vault(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "PreparePersistentVault", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn prepare_vault_for_migration(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "PrepareVaultForMigration", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn add_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "AddAuthFactor", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn update_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "UpdateAuthFactor", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn remove_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "RemoveAuthFactor", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn list_auth_factors(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "ListAuthFactors", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn prepare_async_auth_factor(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "PrepareAsyncAuthFactor", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_recovery_request(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "GetRecoveryRequest", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn reset_application_container(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.UserDataAuthInterface", "ResetApplicationContainer", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }
}

pub trait OrgChromiumArcQuota {
    fn get_arc_disk_features(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_current_space_for_arc_uid(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_current_space_for_arc_gid(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_current_space_for_arc_project_id(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn set_media_rwdata_file_project_id(&self, fd: arg::OwnedFd, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn set_media_rwdata_file_project_inheritance_flag(&self, fd: arg::OwnedFd, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgChromiumArcQuota for blocking::Proxy<'a, C> {

    fn get_arc_disk_features(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.ArcQuota", "GetArcDiskFeatures", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_current_space_for_arc_uid(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.ArcQuota", "GetCurrentSpaceForArcUid", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_current_space_for_arc_gid(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.ArcQuota", "GetCurrentSpaceForArcGid", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_current_space_for_arc_project_id(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.ArcQuota", "GetCurrentSpaceForArcProjectId", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn set_media_rwdata_file_project_id(&self, fd: arg::OwnedFd, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.ArcQuota", "SetMediaRWDataFileProjectId", (fd, request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn set_media_rwdata_file_project_inheritance_flag(&self, fd: arg::OwnedFd, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.ArcQuota", "SetMediaRWDataFileProjectInheritanceFlag", (fd, request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }
}

pub trait OrgChromiumCryptohomePkcs11Interface {
    fn pkcs11_is_tpm_token_ready(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn pkcs11_get_tpm_token_info(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn pkcs11_terminate(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn pkcs11_restore_tpm_tokens(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgChromiumCryptohomePkcs11Interface for blocking::Proxy<'a, C> {

    fn pkcs11_is_tpm_token_ready(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomePkcs11Interface", "Pkcs11IsTpmTokenReady", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn pkcs11_get_tpm_token_info(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomePkcs11Interface", "Pkcs11GetTpmTokenInfo", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn pkcs11_terminate(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomePkcs11Interface", "Pkcs11Terminate", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn pkcs11_restore_tpm_tokens(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomePkcs11Interface", "Pkcs11RestoreTpmTokens", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }
}

pub trait OrgChromiumInstallAttributesInterface {
    fn install_attributes_get(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn install_attributes_set(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn install_attributes_finalize(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn install_attributes_get_status(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_firmware_management_parameters(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn remove_firmware_management_parameters(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn set_firmware_management_parameters(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgChromiumInstallAttributesInterface for blocking::Proxy<'a, C> {

    fn install_attributes_get(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "InstallAttributesGet", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn install_attributes_set(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "InstallAttributesSet", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn install_attributes_finalize(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "InstallAttributesFinalize", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn install_attributes_get_status(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "InstallAttributesGetStatus", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_firmware_management_parameters(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "GetFirmwareManagementParameters", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn remove_firmware_management_parameters(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "RemoveFirmwareManagementParameters", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn set_firmware_management_parameters(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.InstallAttributesInterface", "SetFirmwareManagementParameters", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }
}

pub trait OrgChromiumCryptohomeMiscInterface {
    fn get_system_salt(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn update_current_user_activity_timestamp(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_sanitized_username(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_login_status(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_status_string(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn lock_to_single_user_mount_until_reboot(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn get_rsu_device_id(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
    fn check_health(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error>;
}

impl<'a, T: blocking::BlockingSender, C: ::std::ops::Deref<Target=T>> OrgChromiumCryptohomeMiscInterface for blocking::Proxy<'a, C> {

    fn get_system_salt(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "GetSystemSalt", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn update_current_user_activity_timestamp(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "UpdateCurrentUserActivityTimestamp", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_sanitized_username(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "GetSanitizedUsername", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_login_status(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "GetLoginStatus", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_status_string(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "GetStatusString", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn lock_to_single_user_mount_until_reboot(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "LockToSingleUserMountUntilReboot", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn get_rsu_device_id(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "GetRsuDeviceId", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }

    fn check_health(&self, request: Vec<u8>) -> Result<Vec<u8>, dbus::Error> {
        self.method_call("org.chromium.CryptohomeMiscInterface", "CheckHealth", (request, ))
            .and_then(|r: (Vec<u8>, )| Ok(r.0, ))
    }
}

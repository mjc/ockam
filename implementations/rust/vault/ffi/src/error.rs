use ockam_vault_software::ockam_vault::error::VaultFailError;

// FIXME: This should be removed after introducing common error

pub(crate) fn map_vault_error_err(err: VaultFailError) -> ffi_support::ExternError {
    ffi_support::ExternError::new_error(ffi_support::ErrorCode::new(0 as i32), err.to_string())
}

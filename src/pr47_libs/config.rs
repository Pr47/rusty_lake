use pr47::builtins::closure::Closure;
use pr47::data::Value;
use pr47::data::exception::UncheckedException;
use pr47::data::traits::StaticBase;
use pr47::data::wrapper::{OWN_INFO_READ_MASK, OwnershipInfo};
use pr47::data::tyck::TyckInfoPool;
use pr47::ffi::{DataOption, FFIException, Signature};
use pr47::ffi::sync_fn::{FunctionBase, value_into_mut_ref_noalias, value_into_ref_noalias, VMContext};
use xjbutil::boxed_slice;
use xjbutil::void::Void;

use crate::ServerConfig;
use crate::config::ServerRequestHandler;
use crate::pr47_libs::dangerous_clone_closure;

impl ServerConfig {
    #[cfg(feature = "with-pr47")]
    pub fn add_pr47_handler(&mut self, handler_path: &str, input_closure: &Closure)
        -> Result<(), FFIException>
    {
        let mut closure = unsafe { dangerous_clone_closure(input_closure) };

        for captured_value in closure.capture.iter_mut() {
            unsafe {
                if !captured_value.ownership_info_norm().is_readable() {
                    return Err(FFIException::Right(UncheckedException::OwnershipCheckFailure {
                        object: captured_value.clone(),
                        expected_mask: OWN_INFO_READ_MASK
                    }))
                }
                captured_value.set_ownership_info(OwnershipInfo::SharedToRust);
            }
        }
        self.handlers.push((handler_path.to_string(), ServerRequestHandler::Pr47Function(closure)));
        Ok(())
    }

    #[cfg(feature = "with-pr47")]
    pub fn add_static_file_handler_pr47(
        &mut self,
        handler_path: &str,
        file_path: &str,
        content_type: Option<&str>
    ) {
        self.handlers.push((
            handler_path.to_string(),
            ServerRequestHandler::StaticFile {
                file_path: file_path.to_string(),
                content_type: content_type.or_else(|| {
                    mime_guess::from_path(file_path).first_raw()
                }).map(ToString::to_string)
            }
        ));
    }
}

impl StaticBase<ServerConfig> for Void {}

#[cfg(feature = "with-pr47")]
#[allow(non_camel_case_types)]
pub struct PR47_BIND_add_pr47_handler();

#[cfg(feature = "with-pr47")]
impl FunctionBase for PR47_BIND_add_pr47_handler {
    #[inline(always)] fn signature(tyck_info_pool: &mut TyckInfoPool) -> Signature {
        Signature {
            param_types: boxed_slice![
                <Void as StaticBase<ServerConfig>>::tyck_info(tyck_info_pool),
                <Void as StaticBase<String>>::tyck_info(tyck_info_pool),
                <Void as StaticBase<Closure>>::tyck_info(tyck_info_pool)
            ],
            param_options: boxed_slice![DataOption::MutShare, DataOption::Share, DataOption::Share],
            ret_type: boxed_slice![],
            ret_option: boxed_slice![],
            exceptions: boxed_slice![]
        }
    }

    #[inline(always)] unsafe fn call_rtlc<CTX: VMContext>(
        _context: &mut CTX,
        args: &[Value],
        _rets: &[*mut Value]
    ) -> Result<(), FFIException> {
        let arg0: &mut ServerConfig = value_into_mut_ref_noalias(*args.get_unchecked(0))?;
        let arg1: &String = value_into_ref_noalias(*args.get_unchecked(1))?;
        let arg2: &Closure = value_into_ref_noalias(*args.get_unchecked(2))?;

        arg0.add_pr47_handler(arg1, arg2)
    }

    #[inline(always)] unsafe fn call_unchecked<CTX: VMContext>(
        _context: &mut CTX,
        args: &[Value],
        _rets: &[*mut Value]
    ) -> Result<(), FFIException> {
        let arg0: &mut ServerConfig = &mut *args.get_unchecked(0).get_as_mut_ptr_norm::<_>();
        let arg1: &String = &mut *args.get_unchecked(1).get_as_mut_ptr_norm::<_>();
        let arg2: &Closure = &mut *args.get_unchecked(2).get_as_mut_ptr_norm::<_>();

        arg0.add_pr47_handler(arg1, arg2)
    }
}

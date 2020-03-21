use crate::extn::prelude::*;

pub fn init(interp: &mut Artichoke) -> InitializeResult<()> {
    
    
    let spec = crate::module::Spec::new(interp, "RbConfig", None)?;
    interp.0.borrow_mut().def_module::<RbConfig>(spec);
    
    
    
    let spec = crate::class::Spec::new("StringIO", None, None)?;
    interp.0.borrow_mut().def_class::<StringIO>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPAddr", None, None)?;
    interp.0.borrow_mut().def_class::<IPAddr>(spec);
    
    
    
    let spec = crate::class::Spec::new("IPSocket", None, None)?;
    interp.0.borrow_mut().def_class::<IPSocket>(spec);
    
    
    
    let spec = crate::class::Spec::new("Monitor", None, None)?;
    interp.0.borrow_mut().def_class::<Monitor>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "MonitorMixin", None)?;
    interp.0.borrow_mut().def_module::<MonitorMixin>(spec);
    
    
    
    let spec = crate::module::Spec::new(interp, "Gem", None)?;
    interp.0.borrow_mut().def_module::<Gem>(spec);
    
    
    
    let spec = crate::class::Spec::new("SimpleDelegator", None, None)?;
    interp.0.borrow_mut().def_class::<SimpleDelegator>(spec);
    
    
    
    // Skipping constant CROSS_COMPILING with class NilClass
    
    
    
    let spec = crate::module::Spec::new(interp, "URI", None)?;
    interp.0.borrow_mut().def_module::<URI>(spec);
    
    
    
    let spec = crate::class::Spec::new("Delegator", None, None)?;
    interp.0.borrow_mut().def_class::<Delegator>(spec);
    
    
    
    // Skipping constant RUBYGEMS_ACTIVATION_MONITOR with class Monitor
    
    
    
    interp.def_rb_source_file(
        b"rubygems.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/basic_specification.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/basic_specification.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/bundler_version_finder.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/bundler_version_finder.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/compatibility.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/compatibility.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/core_ext/kernel_gem.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/core_ext/kernel_gem.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/core_ext/kernel_require.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/core_ext/kernel_require.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/core_ext/kernel_warn.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/core_ext/kernel_warn.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/defaults.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/defaults.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/dependency.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/dependency.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/deprecate.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/deprecate.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/errors.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/errors.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/exceptions.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/exceptions.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/platform.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/platform.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/requirement.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/requirement.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/specification.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/specification.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/specification_policy.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/specification_policy.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/stub_specification.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/stub_specification.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/util.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/util.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/util/list.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/util/list.rb"))[..]
    )?;
    
    interp.def_rb_source_file(
        b"rubygems/version.rb",
        &include_bytes!(concat!(env!("OUT_DIR"), "/src/generated/rubygems/version.rb"))[..]
    )?;
    
    Ok(())
}

#[derive(Debug)]
pub struct RbConfig;


#[derive(Debug)]
pub struct StringIO;


#[derive(Debug)]
pub struct IPAddr;


#[derive(Debug)]
pub struct IPSocket;


#[derive(Debug)]
pub struct Monitor;


#[derive(Debug)]
pub struct MonitorMixin;


#[derive(Debug)]
pub struct Gem;


#[derive(Debug)]
pub struct SimpleDelegator;


#[derive(Debug)]
pub struct CROSS_COMPILING;


#[derive(Debug)]
pub struct URI;


#[derive(Debug)]
pub struct Delegator;


#[derive(Debug)]
pub struct RUBYGEMS_ACTIVATION_MONITOR;



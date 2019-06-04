use mruby::gc::GarbageCollection;
use rocket::State;
use std::path::PathBuf;

use crate::request::Request;
use crate::response::Response;
use crate::server::rocket::request;
use crate::server::MountRegistry;
use crate::Error;

#[get("/<path..>")]
#[allow(clippy::needless_pass_by_value)]
pub fn app<'a>(
    path: Option<PathBuf>,
    req: request::Request,
    mounts: State<MountRegistry>,
) -> Result<::rocket::Response<'a>, Error> {
    let _ = path;
    info!("Initializing fresh shared nothing mruby interpreter");
    let mount = mounts
        .0
        .get(req.script_name().as_str())
        .ok_or(Error::NoRoute)?;
    let interp = mount.exec_mode.interpreter(&mount.interp_init)?;
    let _arena = interp.create_arena_savepoint();
    let lock = mount.app.lock().map_err(|_| Error::CannotCreateApp)?;
    let app = lock(&interp)?;
    debug!(
        "Matched Rack adapter: app={} base={} route={}",
        app.name(),
        req.script_name(),
        req.path_info()
    );
    let response = app.call(&req).map(Response::into_rocket)?;
    mount.exec_mode.gc(&interp);
    Ok(response)
}

#[get("/")]
#[allow(clippy::needless_pass_by_value)]
pub fn app_get_root<'a>(
    req: request::Request,
    mounts: State<MountRegistry>,
) -> Result<::rocket::Response<'a>, Error> {
    info!("Initializing fresh shared nothing mruby interpreter");
    let mount = mounts
        .0
        .get(req.script_name().as_str())
        .ok_or(Error::NoRoute)?;
    let interp = mount.exec_mode.interpreter(&mount.interp_init)?;
    let _arena = interp.create_arena_savepoint();
    let lock = mount.app.lock().map_err(|_| Error::CannotCreateApp)?;
    let app = lock(&interp)?;
    debug!(
        "Matched Rack adapter: app={} base={} route={}",
        app.name(),
        req.script_name(),
        req.path_info()
    );
    let response = app.call(&req).map(Response::into_rocket)?;
    mount.exec_mode.gc(&interp);
    Ok(response)
}

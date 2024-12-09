use salvo::prelude::*;
use sea_orm::{ColumnTrait, EntityTrait, NotSet, PaginatorTrait, QueryFilter, QueryOrder};
use sea_orm::ActiveValue::Set;

use crate::AppState;
use crate::common::result::BaseResponse;
use crate::model::system::prelude::SysMenu;
use crate::model::system::sys_menu;
use crate::model::system::sys_menu::ActiveModel;
use crate::vo::system::menu_vo::{*};

// 添加菜单
#[handler]
pub async fn menu_save(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let menu = req.parse_json::<MenuSaveReq>().await.unwrap();
    log::info!("menu_save params: {:?}", &menu);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let sys_menu = ActiveModel {
        id: NotSet,
        status_id: Set(menu.status_id),
        sort: Set(menu.sort),
        parent_id: Set(menu.parent_id),
        menu_name: Set(menu.menu_name),
        menu_url: Set(menu.menu_url.unwrap_or_default()),
        api_url: Set(menu.api_url.unwrap_or_default()),
        menu_icon: Set(menu.icon),
        remark: Set(menu.remark),
        menu_type: Set(menu.menu_type),
        ..Default::default()
    };

    let result = SysMenu::insert(sys_menu).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

// 删除菜单信息
#[handler]
pub async fn menu_delete(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<MenuDeleteReq>().await.unwrap();
    log::info!("menu_delete params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    if SysMenu::find_by_id(item.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return BaseResponse::<String>::err_result_msg(res, "菜单不存在,不能删除!".to_string());
    }

    if SysMenu::find().filter(sys_menu::Column::ParentId.eq(item.id.clone())).count(conn).await.unwrap_or_default() > 0 {
        return BaseResponse::<String>::err_result_msg(res, "有下级菜单,不能直接删除!".to_string());
    }

    let result = SysMenu::delete_by_id(item.id.clone()).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

// 更新菜单
#[handler]
pub async fn menu_update(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let menu = req.parse_json::<MenuUpdateReq>().await.unwrap();
    log::info!("menu_update params: {:?}", &menu);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    if SysMenu::find_by_id(menu.id.clone()).one(conn).await.unwrap_or_default().is_none() {
        return BaseResponse::<String>::err_result_msg(res, "菜单不存在,不能更新!".to_string());
    }

    let sys_menu = ActiveModel {
        id: Set(menu.id),
        status_id: Set(menu.status_id),
        sort: Set(menu.sort),
        parent_id: Set(menu.parent_id),
        menu_name: Set(menu.menu_name),
        menu_url: Set(menu.menu_url.unwrap_or_default()),
        api_url: Set(menu.api_url.unwrap_or_default()),
        menu_icon: Set(menu.icon),
        remark: Set(menu.remark),
        menu_type: Set(menu.menu_type),
        ..Default::default()
    };

    let result = SysMenu::update(sys_menu).exec(conn).await;
    match result {
        Ok(_u) => BaseResponse::<String>::ok_result(res),
        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
    }
}

// 查询菜单
#[handler]
pub async fn menu_list(req: &mut Request, depot: &mut Depot, res: &mut Response) {
    let item = req.parse_json::<MenuListReq>().await;
    log::info!("menu_list params: {:?}", &item);

    let state = depot.obtain::<AppState>().unwrap();
    let conn = &state.conn;

    let mut list_data: Vec<MenuListData> = Vec::new();

    for menu in SysMenu::find().order_by_asc(sys_menu::Column::Sort).all(conn).await.unwrap_or_default() {
        list_data.push(MenuListData {
            id: menu.id,
            sort: menu.sort,
            status_id: menu.status_id,
            parent_id: menu.parent_id,
            menu_name: menu.menu_name.clone(),
            label: menu.menu_name,
            menu_url: menu.menu_url,
            icon: menu.menu_icon.unwrap_or_default(),
            api_url: menu.api_url,
            remark: menu.remark.unwrap_or_default(),
            menu_type: menu.menu_type,
            create_time: menu.create_time.to_string(),
            update_time: menu.update_time.to_string(),
        })
    }

    BaseResponse::<Vec<MenuListData>>::ok_result_page(res, list_data, 0)
}

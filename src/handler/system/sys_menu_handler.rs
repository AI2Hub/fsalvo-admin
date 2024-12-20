// author：刘飞华
// createTime：2024/12/20 14:19:08

use diesel::associations::HasTable;
use diesel::sql_types::*;
use diesel::{sql_query, ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error};
use salvo::prelude::*;
use salvo::{Request, Response};

use crate::common::result::BaseResponse;
use crate::model::system::sys_menu_model::*;
use crate::schema::sys_menu::dsl::sys_menu;
use crate::schema::sys_menu::*;
use crate::vo::system::sys_menu_vo::*;
use crate::RB;

/*
 *添加菜单信息
 *author：刘飞华
 *date：2024/12/20 14:19:08
 */
#[handler]
pub async fn add_sys_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<AddMenuReq>().await.unwrap();
    log::info!("add sys_menu params: {:?}", &item);

    let add_sys_menu_param = AddSysMenu {
        menu_name: item.menu_name,       //菜单名称
        menu_type: item.menu_type,       //菜单类型(1：目录   2：菜单   3：按钮)
        status_id: item.status_id,       //状态(1:正常，0:禁用)
        sort: item.sort,                 //排序
        parent_id: item.parent_id,       //父ID
        menu_url: item.menu_url,         //路由路径
        api_url: item.api_url,           //接口URL
        menu_icon: item.menu_icon,       //菜单图标
        remark: item.remark,             //备注, //创建时间
        create_time: Default::default(), //修改时间
        update_time: Default::default(),
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::insert_into(sys_menu::table())
                .values(add_sys_menu_param)
                .execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(res, err.to_string())
        }
    }
}

/*
 *删除菜单信息
 *author：刘飞华
 *date：2024/12/20 14:19:08
 */
#[handler]
pub async fn delete_sys_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<DeleteMenuReq>().await.unwrap();
    log::info!("delete sys_menu params: {:?}", &item);
    match &mut RB.clone().get() {
        Ok(conn) => {
            match sys_menu
                .filter(parent_id.eq(&item.id))
                .count()
                .get_result::<i64>(conn)
            {
                Ok(count) => {
                    if count > 0 {
                        error!("err:{}", "有下级菜单,不能直接删除".to_string());
                        return BaseResponse::<String>::err_result_msg(
                            res,
                            "有下级菜单,不能直接删除".to_string(),
                        );
                    }
                    let result = diesel::delete(sys_menu.filter(id.eq(&item.id))).execute(conn);
                    match result {
                        Ok(_u) => BaseResponse::<String>::ok_result(res),
                        Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
                    };
                }
                Err(err) => {
                    error!("err:{}", err.to_string());
                    BaseResponse::<String>::err_result_msg(res, err.to_string())
                }
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(res, err.to_string())
        }
    }
}

/*
 *更新菜单信息
 *author：刘飞华
 *date：2024/12/20 14:19:08
 */
#[handler]
pub async fn update_sys_menu(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateMenuReq>().await.unwrap();
    log::info!("update sys_menu params: {:?}", &item);

    let update_sys_menu_param = UpdateSysMenu {
        id: item.id,                     //主键
        menu_name: item.menu_name,       //菜单名称
        menu_type: item.menu_type,       //菜单类型(1：目录   2：菜单   3：按钮)
        status_id: item.status_id,       //状态(1:正常，0:禁用)
        sort: item.sort,                 //排序
        parent_id: item.parent_id,       //父ID
        menu_url: item.menu_url,         //路由路径
        api_url: item.api_url,           //接口URL
        menu_icon: item.menu_icon,       //菜单图标
        remark: item.remark,             //备注, //创建时间
        create_time: Default::default(), //修改时间
        update_time: Default::default(),
    };

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_menu)
                .filter(id.eq(&item.id))
                .set(update_sys_menu_param)
                .execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(res, err.to_string())
        }
    }
}

/*
 *更新菜单信息状态
 *author：刘飞华
 *date：2024/12/20 14:19:08
 */
#[handler]
pub async fn update_sys_menu_status(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<UpdateMenuStatusReq>().await.unwrap();
    log::info!("update sys_menu_status params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let result = diesel::update(sys_menu)
                .filter(id.eq_any(&item.ids))
                .set(status_id.eq(item.status))
                .execute(conn);
            match result {
                Ok(_u) => BaseResponse::<String>::ok_result(res),
                Err(err) => BaseResponse::<String>::err_result_msg(res, err.to_string()),
            };
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<String>::err_result_msg(res, err.to_string())
        }
    }
}

/*
 *查询菜单信息详情
 *author：刘飞华
 *date：2024/12/20 14:19:08
 */
#[handler]
pub async fn query_sys_menu_detail(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryMenuDetailReq>().await.unwrap();

    log::info!("query sys_menu_detail params: {:?}", &item);

    match &mut RB.clone().get() {
        Ok(conn) => {
            let sys_menu_sql = sql_query("SELECT * FROM sys_menu WHERE id = ?");
            let result = sys_menu_sql
                .bind::<Bigint, _>(&item.id)
                .get_result::<SysMenu>(conn);
            if let Ok(x) = result {
                let data = QueryMenuDetailResp {
                    id: x.id,                               //主键
                    menu_name: x.menu_name,                 //菜单名称
                    menu_type: x.menu_type,                 //菜单类型(1：目录   2：菜单   3：按钮)
                    status_id: x.status_id,                 //状态(1:正常，0:禁用)
                    sort: x.sort,                           //排序
                    parent_id: x.parent_id,                 //父ID
                    menu_url: x.menu_url,                   //路由路径
                    api_url: x.api_url,                     //接口URL
                    menu_icon: x.menu_icon,                 //菜单图标
                    remark: x.remark.unwrap_or_default(),   //备注
                    create_time: x.create_time.to_string(), //创建时间
                    update_time: x.update_time.to_string(), //修改时间
                };

                BaseResponse::<QueryMenuDetailResp>::ok_result_data(res, data)
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<QueryMenuDetailResp>::err_result_data(
                res,
                QueryMenuDetailResp::new(),
                err.to_string(),
            )
        }
    }
}

/*
 *查询菜单信息列表
 *author：刘飞华
 *date：2024/12/20 14:19:08
 */
#[handler]
pub async fn query_sys_menu_list(req: &mut Request, res: &mut Response) {
    let item = req.parse_json::<QueryMenuListReq>().await.unwrap();
    log::info!("query sys_menu_list params: {:?}", &item);

    let mut query = sys_menu::table().into_boxed();

    query = query.order(sort.asc());

    debug!(
        "SQL:{}",
        diesel::debug_query::<diesel::mysql::Mysql, _>(&query).to_string()
    );

    match &mut RB.clone().get() {
        Ok(conn) => {
            let mut sys_menu_list_data: Vec<MenuListDataResp> = Vec::new();
            if let Ok(list) = query.load::<SysMenu>(conn) {
                for x in list {
                    sys_menu_list_data.push(MenuListDataResp {
                        id: x.id,                               //主键
                        menu_name: x.menu_name,                 //菜单名称
                        menu_type: x.menu_type, //菜单类型(1：目录   2：菜单   3：按钮)
                        status_id: x.status_id, //状态(1:正常，0:禁用)
                        sort: x.sort,           //排序
                        parent_id: x.parent_id, //父ID
                        menu_url: x.menu_url,   //路由路径
                        api_url: x.api_url,     //接口URL
                        menu_icon: x.menu_icon, //菜单图标
                        remark: x.remark.unwrap_or_default(), //备注
                        create_time: x.create_time.to_string(), //创建时间
                        update_time: x.update_time.to_string(), //修改时间
                    })
                }
                BaseResponse::<Vec<MenuListDataResp>>::ok_result_page(res, sys_menu_list_data, 10)
            }
        }
        Err(err) => {
            error!("err:{}", err.to_string());
            BaseResponse::<Vec<MenuListDataResp>>::err_result_page(
                res,
                MenuListDataResp::new(),
                err.to_string(),
            )
        }
    }
}

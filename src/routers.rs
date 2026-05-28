use crate::handlers::{auth, receiver, user};
use crate::middlewares::api_auth;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api")
            .service(
                web::scope("/default_receiver").service(
                    web::resource("")
                        .route(web::get().to(receiver::get_default_receiver))
                ),
            )
            .service(
                web::scope("/auth").service(
                    web::resource("/token")
                        .route(web::post().to(auth::login))
                        .route(web::delete().to(auth::logout).wrap(api_auth::CheckLogin)),
                ),
            )
            .service(
                web::scope("/user")
                    .wrap(api_auth::CheckLogin)
                    .service(web::resource("/profile").route(web::get().to(user::get_profile)))
                    .service(
                        web::resource("/password").route(web::put().to(user::update_password)),
                    ),
            )
            .service(
                web::scope("/admin")
                    .wrap(api_auth::CheckAdminLogin)
                    .service(
                        web::scope("/users")
                            .service(
                                web::resource("")
                                    .route(web::get().to(user::list_users))
                                    .route(web::post().to(user::add_user)),
                            )
                            .service(web::scope("/{user_id}").service(
                                web::resource("").route(web::put().to(user::update_user)),
                            )),
                    )
                    .service(
                        web::scope("/receivers")
                            .service(
                                web::resource("")
                                    .route(web::get().to(receiver::list_receivers))
                                    .route(web::post().to(receiver::add_receiver)),
                            )
                            .service(web::scope("/{receiver_id}").service(
                                web::resource("")
                                    .route(web::put().to(receiver::update_receiver))
                                    .route(web::delete().to(receiver::delete_receiver)),
                            )),
                    )
            )
            // .service(
            //     web::scope("/omics")
            //         .wrap(api_auth::CheckLogin)
            //         .service(web::resource("/token").route(web::get().to(omics::oss_token)))
            //         .service(
            //             web::scope("/quartet")
            //                 .service(
            //                     web::scope("/dna")
            //                         .service(
            //                             web::resource("")
            //                                 .route(web::post().to(omics::add_dna))
            //                                 .route(web::get().to(omics::list_dna)),
            //                         )
            //                         .service(
            //                             web::resource("/report")
            //                                 .route(web::post().to(omics::generate_dna_report)),
            //                         )
            //                         .service(web::scope("/{dna_id}").service(
            //                             web::resource("").route(web::put().to(omics::update_dna)),
            //                         )),
            //                 )
            //         )
            // ),
    );
}

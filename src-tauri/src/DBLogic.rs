use sea_orm::{entity, ConnectOptions, Identity, ConnectionTrait, EntityTrait,Database,ActiveModelTrait, DatabaseConnection, DbErr, Schema, Set};
use sea_orm::sea_query::table::TableCreateStatement;
use sea_orm::entity::*;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "passwords")] 
pub struct Model {
    #[sea_orm(primary_key)] 
    pub id: i32,
    #[sea_orm(column_name = "Service_Name")]  // Явно указываем имя столбца
    pub service_name: String,
    #[sea_orm(column_name = "Pass")]  // Явно указываем имя столбца
    pub pass: String,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}


//pub async fn test_pass(db: &DatabaseConnection) -> Result<String, DbErr> {
   // let password = crate::passwords::ActiveModel {
    //    service_name: Set("Сервис".to_owned()),
  //      pass: Set("123 пароль".to_owned()),
    //    ..Default::default() 
  //  };
    
    //let res = password.insert(db).await?;
  //  println!("Пароль добавлен с ID: {}", res.id);
    
  //  Ok(format!("Пароль добавлен с ID: {}", res.id))
//}
#[tokio::main]
pub async fn initialize_db_context() -> Result<DatabaseConnection, sea_orm::DbErr> {
    let db = Database::connect("sqlite://PasstoolDB.db?mode=rwc").await?;
    
    let builder = db.get_database_backend();
    let schema = Schema::new(builder);
    let stmt: TableCreateStatement = schema.create_table_from_entity(Entity);
    
    db.execute(builder.build(&stmt)).await?;
    println!("Таблица 'passwords' создана успешно");
   // test_pass(&db).await?;
    
    Ok(db)
}
/*
Base
datasource db {
  provider             = "mysql"
  url                  = env("DATABASE_URL")
  referentialIntegrity = "prisma"
}

generator client {
  provider        = "prisma-client-js"
  previewFeatures = ["referentialIntegrity"]
}

generator nexusPrisma {
  provider        = "nexus-prisma"
  previewFeatures = ["referentialIntegrity"]
}

enum ContentType {
  TEXT
  LINK
  IMAGE
  VIDEO
  EMOJI
}

model AllowedWritingRole {
  id     Int     @id @default(autoincrement())
  create Boolean @default(false)
  read   Boolean @default(false)
  update Boolean @default(false)
  delete Boolean @default(false)

  Boards_defaultBoardRole  Board[] @relation("Boards_defaultBoardRole")
  Boards_defaultPostRole   Board[] @relation("Boards_defaultPostRole")
  Boards_defaultThreadRole Board[] @relation("Boards_defaultThreadRole")
  Boards_defaultReplyRole  Board[] @relation("Boards_defaultReplyRole")

  Posts_defaultPostRole   Post[] @relation("Posts_defaultPostRole")
  Posts_defaultThreadRole Post[] @relation("Posts_defaultThreadRole")
  Posts_defaultReplyRole  Post[] @relation("Posts_defaultReplyRole")

  SystemAdministratorRoles_boardRole  SystemAdministratorRole[] @relation("SystemAdministratorRoles_boardRole")
  SystemAdministratorRoles_postRole   SystemAdministratorRole[] @relation("SystemAdministratorRoles_postRole")
  SystemAdministratorRoles_threadRole SystemAdministratorRole[] @relation("SystemAdministratorRoles_threadRole")
  SystemAdministratorRoles_replyRole  SystemAdministratorRole[] @relation("SystemAdministratorRoles_replyRole")

  BoardRoles_boardRole  BoardRole[] @relation("BoardRoles_boardRole")
  BoardRoles_postRole   BoardRole[] @relation("BoardRoles_postRole")
  BoardRoles_threadRole BoardRole[] @relation("BoardRoles_threadRole")
  BoardRoles_replyRole  BoardRole[] @relation("BoardRoles_replyRole")

  PostRoles_postRole   PostRole[] @relation("PostRoles_postRole")
  PostRoles_threadRole PostRole[] @relation("PostRoles_threadRole")
  PostRoles_replyRole  PostRole[] @relation("PostRoles_replyRole")
}

model User {
  id                  Int                @id @default(autoincrement())
  createdAt           DateTime           @default(now())
  token               String             @unique
  thirdPartyAPITokens ThirdPartyAPIKey[] @relation()
  personas            Persona[]
}

model Persona {
  id                          Int                       @id @default(autoincrement())
  createdAt                   DateTime                  @default(now())
  name                        String                    @unique @db.VarChar(25)
  screenName                  String                    @db.VarChar(30)
  iconUrl                     String                    @db.Text
  FollowingBoards             FollowingBoard[]
  modelatingBoards            Board[]                   @relation()
  followingUsers              PersonaRelation[]         @relation("destPersona")
  followedUsers               PersonaRelation[]         @relation("srcPersona")
  votesOnPosts                VoteOnPost[]
  votesOnThreads              VoteOnThread[]
  votesOnReplies              VoteOnReply[]
  user                        User                      @relation(fields: [userId], references: [id])
  userId                      Int
  posts                       Post[]
  threads                     Thread[]
  replies                     Reply[]
  systemAdministratorRoles    SystemAdministratorRole[]
  boardRoles                  BoardRole[]
  postRoles                   PostRole[]
  bot                         Bot?
  notificationEvent_senders   NotificationEvent[]       @relation("NotificationEvent_sender")
  notificationEvent_receivers NotificationEvent[]       @relation("NotificationEvent_receiver")
  directMesswage_senders      DirectMessage[]           @relation("DirectMessage_sender")
  direectMessages_receivers   DirectMessage[]           @relation("DirectMessage_receiver")
  PersonaProfile              PersonaProfile?
}

model PersonaRelation {
  id            Int      @id @default(autoincrement())
  createdAt     DateTime @default(now())
  srcPersonaId  Int
  srcPersona    Persona  @relation("srcPersona", fields: [srcPersonaId], references: [id])
  destPersonaId Int
  destPersona   Persona  @relation("destPersona", fields: [destPersonaId], references: [id])
}

model VoteOnPost {
  id          Int      @id @default(autoincrement())
  createdAt   DateTime @default(now())
  createdById Int
  createdBy   Persona  @relation(fields: [createdById], references: [id])
  weight      Int
}

model VoteOnThread {
  id          Int      @id @default(autoincrement())
  createdAt   DateTime @default(now())
  createdById Int
  createdBy   Persona  @relation(fields: [createdById], references: [id])
  weight      Int
}

model VoteOnReply {
  id          Int      @id @default(autoincrement())
  createdAt   DateTime @default(now())
  createdById Int
  createdBy   Persona  @relation(fields: [createdById], references: [id])
  weight      Int
}

model Board {
  id                  String             @id @db.Char(26)
  title               String             @unique @db.VarChar(30)
  createdAt           DateTime           @default(now())
  deletedAt           DateTime?
  description         String             @db.VarChar(2000)
  moderators          Persona[]          @relation()
  followedBy          FollowingBoard[]
  posts               Post[]
  threads             Thread[]
  roles               BoardRole[]
  defaultBoardRoleId  Int
  defaultBoardRole    AllowedWritingRole @relation("Boards_defaultBoardRole", fields: [defaultBoardRoleId], references: [id])
  defaultPostRoleId   Int
  defaultPostRole     AllowedWritingRole @relation("Boards_defaultPostRole", fields: [defaultPostRoleId], references: [id])
  defaultThreadRoleId Int
  defaultThreadRole   AllowedWritingRole @relation("Boards_defaultThreadRole", fields: [defaultThreadRoleId], references: [id])
  defaultReplyRoleId  Int
  defaultReplyRole    AllowedWritingRole @relation("Boards_defaultReplyRole", fields: [defaultReplyRoleId], references: [id])
}

model FollowingBoard {
  id        String    @id @db.Char(26)
  createdAt DateTime  @default(now())
  deletedAt DateTime?
  board     Board     @relation(fields: [boardId], references: [id])
  boardId   String
  persona   Persona   @relation(fields: [personaId], references: [id])
  personaId Int

  @@unique([boardId, personaId, deletedAt], name: "followingBoardUnique")
}

model Post {
  id                  String             @id @db.Char(26)
  createdAt           DateTime           @default(now())
  deletedAt           DateTime?
  title               String             @db.VarChar(50)
  contentType         ContentType
  content             String             @db.VarChar(2000)
  threads             Thread[]
  board               Board?              @relation(fields: [boardId], references: [id])
  boardId             String?
  persona             Persona            @relation(fields: [personaId], references: [id])
  personaId           Int
  roles               PostRole[]
  defaultPostRoleId   Int
  defaultPostRole     AllowedWritingRole @relation("Posts_defaultPostRole", fields: [defaultPostRoleId], references: [id])
  defaultThreadRoleId Int
  defaultThreadRole   AllowedWritingRole @relation("Posts_defaultThreadRole", fields: [defaultThreadRoleId], references: [id])
  defaultReplyRoleId  Int
  defaultReplyRole    AllowedWritingRole @relation("Posts_defaultReplyRole", fields: [defaultReplyRoleId], references: [id])
}

model Thread {
  id          String      @id @db.Char(26)
  createdAt   DateTime    @default(now())
  deletedAt   DateTime?
  content     String      @db.VarChar(500)
  contentType ContentType
  replies     Reply[]
  board       Board?       @relation(fields: [boardId], references: [id])
  boardId     String?
  postId      String
  Post        Post        @relation(fields: [postId], references: [id])
  persona     Persona     @relation(fields: [personaId], references: [id])
  personaId   Int
}

model Reply {
  id          String      @id @db.Char(26)
  contentType ContentType
  content     String      @db.VarChar(500)
  createdAt   DateTime    @default(now())
  deletedAt   DateTime?
  thread      Thread      @relation(fields: [threadId], references: [id])
  threadId    String
  persona     Persona     @relation(fields: [personaId], references: [id])
  personaId   Int
}

model UploadedImage {
  id       String @id @db.Char(26)
  parentId String @db.Char(26)
  fileUrl  String @db.Text
}

model SystemAdministratorRole {
  id           String             @id @db.Char(26)
  personas     Persona[]
  allowAll     Boolean            @default(false)
  roleManager  Boolean            @default(false)
  boardRoleId  Int
  boardRole    AllowedWritingRole @relation("SystemAdministratorRoles_boardRole", fields: [boardRoleId], references: [id])
  postRoleId   Int
  postRole     AllowedWritingRole @relation("SystemAdministratorRoles_postRole", fields: [postRoleId], references: [id])
  threadRoleId Int
  threadRole   AllowedWritingRole @relation("SystemAdministratorRoles_threadRole", fields: [threadRoleId], references: [id])
  replyRoleId  Int
  replyRole    AllowedWritingRole @relation("SystemAdministratorRoles_replyRole", fields: [replyRoleId], references: [id])
}

model BoardRole {
  id           String             @id @db.Char(26)
  personas     Persona[]
  boards       Board[]
  allowAll     Boolean            @default(false)
  roleManager  Boolean            @default(false)
  boardRoleId  Int
  boardRole    AllowedWritingRole @relation("BoardRoles_boardRole", fields: [boardRoleId], references: [id])
  postRoleId   Int
  postRole     AllowedWritingRole @relation("BoardRoles_postRole", fields: [postRoleId], references: [id])
  threadRoleId Int
  threadRole   AllowedWritingRole @relation("BoardRoles_threadRole", fields: [threadRoleId], references: [id])
  replyRoleId  Int
  replyRole    AllowedWritingRole @relation("BoardRoles_replyRole", fields: [replyRoleId], references: [id])
}

model PostRole {
  id           String             @id @db.Char(26)
  personas     Persona[]
  posts        Post[]
  allowAll     Boolean            @default(false)
  roleManager  Boolean            @default(false)
  postRoleId   Int
  postRole     AllowedWritingRole @relation("PostRoles_postRole", fields: [postRoleId], references: [id])
  threadRoleId Int
  threadRole   AllowedWritingRole @relation("PostRoles_threadRole", fields: [threadRoleId], references: [id])
  replyRoleId  Int
  replyRole    AllowedWritingRole @relation("PostRoles_replyRole", fields: [replyRoleId], references: [id])
}

enum ThirdPartyAPIKeyType {
  BOT
  USER
}

model ThirdPartyAPIKey {
  id        String               @id @db.Char(26)
  type      ThirdPartyAPIKeyType
  token     String               @unique
  userId    Int
  user      User                 @relation(fields: [userId], references: [id])
  bot       Bot?
  createdAt DateTime             @default(now())
  revokedAt DateTime?
}

model Bot {
  id                 String           @id @db.Char(26)
  persona            Persona          @relation(fields: [personaId], references: [id])
  personaId          Int              @unique
  thirdPartyAPIKey   ThirdPartyAPIKey @relation(fields: [thirdPartyAPIKeyId], references: [id])
  thirdPartyAPIKeyId String           @unique @db.Char(26)
}

enum NotificationEventType {
  NEW_FOLLOWER
  LIKE
  COMMENT
  DIRECT_MESSAGE
}

model NotificationEvent {
  id         String                @id @db.Char(26)
  sender     Persona               @relation("NotificationEvent_sender", fields: [senderId], references: [id])
  senderId   Int
  receiver   Persona               @relation("NotificationEvent_receiver", fields: [receiverId], references: [id])
  receiverId Int
  type       NotificationEventType
  createdAt  DateTime              @default(now())
  readAt     DateTime?
}

model DirectMessage {
  id               String   @id @db.Char(26)
  sender           Persona  @relation("DirectMessage_sender", fields: [senderId], references: [id])
  senderId         Int
  receiver         Persona  @relation("DirectMessage_receiver", fields: [receiverId], references: [id])
  receiverId       Int
  encryptedContent String   @db.Text
  iv String    @db.Text
  createdAt        DateTime @default(now())
}

model PersonaProfile {
  id          String   @id @db.Char(26)
  persona     Persona  @relation(fields: [personaId], references: [id])
  personaId   Int      @unique
  createdAt   DateTime @default(now())
  bio         String   @db.Text
  headerImage String
}


*/

use async_graphql::*;
use chrono::DateTime;
use chrono::NaiveDateTime;
use chrono::Utc;
use entities::sea_orm_active_enums::ContentType;
use entities::*;
use sea_orm::ColumnTrait;
use sea_orm::DatabaseConnection;
use sea_orm::DbErr;
use sea_orm::EntityTrait;
use sea_orm::ModelTrait;
use sea_orm::QueryFilter;
use std::string::String;

// Model: User
#[derive(Clone, Debug, PartialEq, Eq, SimpleObject)]
#[graphql(complex)]
pub struct User {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub token: String,
}

#[ComplexObject]
impl User {
    async fn personas(&self, context: &Context<'_>) -> Result<Vec<Persona>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let personas = entities::persona::Entity::find()
            .filter(entities::persona::Column::UserId.eq(self.id.clone()))
            .all(db)
            .await?;
        Ok(personas.into_iter().map(Persona::from).collect())
    }
}

impl From<entities::user::Model> for User {
    fn from(model: entities::user::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            token: model.token,
        }
    }
}
// Model: Persona
#[derive(Clone, Debug, SimpleObject)]
pub struct Persona {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub screen_name: String,
    pub icon_url: String,
    pub user_id: i32,
}

impl From<entities::persona::Model> for Persona {
    fn from(model: entities::persona::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            name: model.name,
            screen_name: model.screen_name,
            icon_url: model.icon_url,
            user_id: model.user_id,
        }
    }
}

// Model: Post
#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct Post {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub title: String,
    pub content_type: String,
    pub content: String,
    pub board_id: String,
    pub persona_id: i32,
}

impl From<entities::post::Model> for Post {
    fn from(model: entities::post::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            deleted_at: model.deleted_at,
            title: model.title,
            content_type: model.content_type.to_string(),
            content: model.content,
            board_id: model.board_id,
            persona_id: model.persona_id,
        }
    }
}

#[ComplexObject]
impl Post {
    async fn threads(&self, context: &Context<'_>) -> Result<Vec<Thread>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let threads = entities::thread::Entity::find()
            .filter(entities::thread::Column::PostId.eq(self.id.clone()))
            .all(db)
            .await?;
        Ok(threads.into_iter().map(Thread::from).collect())
    }
}
// Model: Thread
#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct Thread {
    pub id: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub content: String,
    pub content_type: String,
    pub board_id: String,
    pub post_id: String,
    pub persona_id: i32,
}

impl From<entities::thread::Model> for Thread {
    fn from(model: entities::thread::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            deleted_at: model.deleted_at,
            content: model.content,
            content_type: model.content_type.to_string(),
            board_id: model.board_id,
            post_id: model.post_id,
            persona_id: model.persona_id,
        }
    }
}

#[ComplexObject]
impl Thread {
    async fn replies(&self, context: &Context<'_>) -> Result<Vec<Reply>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let replies = entities::reply::Entity::find()
            .filter(entities::reply::Column::ThreadId.eq(self.id.clone()))
            .all(db)
            .await?;
        Ok(replies.into_iter().map(Reply::from).collect())
    }
}

// Model: Reply
#[derive(Clone, Debug, SimpleObject)]
pub struct Reply {
    pub id: String,
    pub content_type: String,
    pub content: String,
    pub created_at: NaiveDateTime,
    pub deleted_at: Option<NaiveDateTime>,
    pub thread_id: String,
    pub persona_id: i32,
}
impl From<entities::reply::Model> for Reply {
    fn from(model: entities::reply::Model) -> Self {
        Self {
            id: model.id,
            created_at: model.created_at,
            deleted_at: model.deleted_at,
            content: model.content,
            content_type: model.content_type.to_string(),
            thread_id: model.thread_id,
            persona_id: model.persona_id,
        }
    }
}

// Model: Bot
#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct Bot {
    pub id: String,
    pub persona_id: i32,
    pub third_party_api_key_id: String,
}

impl From<entities::bot::Model> for Bot {
    fn from(model: entities::bot::Model) -> Self {
        Self {
            id: model.id,
            persona_id: model.persona_id,
            third_party_api_key_id: model.third_party_api_key_id,
        }
    }
}

#[ComplexObject]
impl Bot {
    async fn persona(&self, context: &Context<'_>) -> Result<Option<Persona>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let persona = entities::persona::Entity::find()
            .filter(entities::persona::Column::Id.eq(self.persona_id.clone()))
            .one(db)
            .await?;
        Ok(persona.map(Persona::from))
    }

    async fn third_party_api_key(
        &self,
        context: &Context<'_>,
    ) -> Result<Option<ThirdPartyAPIKey>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let third_party_api_key = entities::third_party_api_key::Entity::find()
            .filter(
                entities::third_party_api_key::Column::Id.eq(self.third_party_api_key_id.clone()),
            )
            .one(db)
            .await?;
        Ok(third_party_api_key.map(ThirdPartyAPIKey::from))
    }
}

// Model: THirdPartyAPIKey
#[derive(Clone, Debug, SimpleObject)]
#[graphql(complex)]
pub struct ThirdPartyAPIKey {
    pub id: String,
    pub r#type: String,
    pub token: String,
    pub created_at: NaiveDateTime,
    pub revoked_at: Option<NaiveDateTime>,
    pub user_id: i32,
}

#[ComplexObject]
impl ThirdPartyAPIKey {
    // user
    async fn user(&self, context: &Context<'_>) -> Result<Option<User>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let user = entities::user::Entity::find()
            .filter(entities::user::Column::Id.eq(self.user_id.clone()))
            .one(db)
            .await?;
        Ok(user.map(User::from))
    }
    // bot
    async fn bot(&self, context: &Context<'_>) -> Result<Option<Bot>, DbErr> {
        let db = context.data::<DatabaseConnection>().unwrap();
        let bot = entities::bot::Entity::find()
            .filter(entities::bot::Column::ThirdPartyApiKeyId.eq(self.id.clone()))
            .one(db)
            .await?;
        Ok(bot.map(Bot::from))
    }
}

impl From<entities::third_party_api_key::Model> for ThirdPartyAPIKey {
    fn from(model: entities::third_party_api_key::Model) -> Self {
        Self {
            id: model.id,
            r#type: model.r#type.to_string(),
            token: model.token,
            created_at: model.created_at,
            revoked_at: model.revoked_at,
            user_id: model.user_id,
        }
    }
}

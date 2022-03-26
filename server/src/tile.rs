use super::{
    error::{Error, Result},
    models::{NewTile, Tile},
    schema::tiles,
};
use diesel::{pg::PgConnection, prelude::*};
use nanoid::nanoid;

pub fn create_tile(connect: &PgConnection, owner: &str, name: &str, r#type: i16) -> Result<Tile> {
    let id = nanoid!();
    let tile = NewTile {
        id: id.as_str(),
        owner: owner,
        name: name,
        type_: r#type,
    };

    diesel::insert_into(tiles::table)
        .values(&tile)
        .get_result::<Tile>(connect)
        .map_err(|_| Error::TileCreationFailure)
}

pub fn update_tile(connect: &PgConnection, id: &str, completion: &[u8]) -> Result<Tile> {
    diesel::update(tiles::table.find(id))
        .set(tiles::completion.eq(completion))
        .get_result::<Tile>(connect)
        .map_err(|_| Error::TileCreationFailure)
}

pub fn get_tile(connect: &PgConnection, id: &str) -> Result<Tile> {
    tiles::table
        .find(id)
        .get_result(connect)
        .map_err(|_| Error::TileFetchFailure)
}

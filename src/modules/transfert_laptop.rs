use async_graphql::ErrorExtensions;
use diesel::prelude::*;
use time::{OffsetDateTime, PrimitiveDateTime};
use uuid::Uuid;

use crate::{
    models::{
        magasin_stock::MagasinStockEntry, sortie_stock::SortieStock,
        transfert_en_cours::TransfertPendingEntry,
    },
    DbPoolConnection,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("the given laptop was already transfered to {}", .0.to_string())]
    AlreadyTransfered(Uuid),
    #[error("the given laptop is not in stock")]
    LaptopNotInStock,
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{self}")).extend_with(|_err, e| match self {
            Error::AlreadyTransfered(er) => {
                e.set("code", "LAPTOP_ALREADY_TRANSFERED");
                e.set("to", er.to_string());
            }
            Error::LaptopNotInStock => {
                e.set("code", "LAPTOP_NOT_IN_STOCK");
            }
        })
    }
}

pub struct TransfertLaptopModule<'a> {
    pub laptop_id: Uuid,
    pub connection: &'a mut DbPoolConnection,
}

impl<'a> TransfertLaptopModule<'a> {
    pub fn is_transfered_to(&mut self, point_vente_id: Uuid) -> crate::Result<Option<Uuid>> {
        use crate::views::v_transfert_en_cours::dsl::*;
        Ok(v_transfert_en_cours
            .select(TransfertPendingEntry::as_select())
            .filter(point_vente.eq(point_vente_id))
            .limit(1)
            .load(self.connection)?
            .into_iter()
            .map(|e: TransfertPendingEntry| e.point_vente)
            .next())
    }
    pub fn get_stock_entry(&mut self) -> crate::Result<MagasinStockEntry> {
        use crate::views::v_magasin_stock::dsl::*;
        Ok(v_magasin_stock
            .filter(laptop.eq(self.laptop_id))
            .select(MagasinStockEntry::as_select())
            .get_result(self.connection)?)
    }
    fn transfert<D: Into<PrimitiveDateTime>>(
        &mut self,
        point_vente_id: Uuid,
        date: Option<D>,
    ) -> crate::Result<SortieStock> {
        let entry = self.get_stock_entry()?;
        use crate::schema::sortie_stock::dsl::*;
        let date: PrimitiveDateTime = date.map(|d| d.into()).unwrap_or_else(|| {
            let now = OffsetDateTime::now_utc();
            PrimitiveDateTime::new(now.date(), now.time())
        });
        let to_use = SortieStock {
            id_sortie_stock: Uuid::new_v4(),
            date_sortie: date,
            point_vente: point_vente_id,
            entree_stock: entry.id_entree_stock,
        };
        Ok(diesel::insert_into(sortie_stock)
            .values(&to_use)
            .returning(SortieStock::as_returning())
            .get_result(self.connection)?)
    }
    pub fn transfert_to<D: Into<PrimitiveDateTime>>(
        &mut self,
        point_vente_id: Uuid,
        date: Option<D>,
    ) -> crate::Result<TransfertPendingEntry> {
        if let Some(pv) = self.is_transfered_to(point_vente_id)? {
            return Err(crate::Error::TransfertLaptopModule(
                Error::AlreadyTransfered(pv),
            ));
        }
        let sortie_entry = self.transfert(point_vente_id, date)?;
        use crate::views::v_transfert_en_cours::dsl::*;
        Ok(v_transfert_en_cours
            .filter(id_demande.eq(sortie_entry.id_sortie_stock))
            .select(TransfertPendingEntry::as_select())
            .get_result(self.connection)?)
    }
}

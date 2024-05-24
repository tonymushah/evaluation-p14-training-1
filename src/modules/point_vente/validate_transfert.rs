use async_graphql::ErrorExtensions;
use diesel::prelude::*;
use time::PrimitiveDateTime;
use uuid::Uuid;

use crate::{
    models::{
        entree_stock_point_vente::EntreeStockPointVente, point_vente_stock::PointVenteStockEntry,
        transfert_en_cours::TransfertPendingEntry,
    },
    DbPoolConnection,
};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("this {} laptop is not in the waiting list", .0.to_string())]
    NotInWaitingList(Uuid),
}

impl ErrorExtensions for Error {
    fn extend(&self) -> async_graphql::Error {
        async_graphql::Error::new(format!("{}", self)).extend_with(|_err, e| match self {
            Error::NotInWaitingList(_) => e.set("code", "LAPTOP_NOT_IN_THE_WAITING_LIST"),
        })
    }
}

pub struct ValidateTransfertModule<'a> {
    pub point_vente: Uuid,
    pub conn: &'a mut DbPoolConnection,
}

impl<'a> ValidateTransfertModule<'a> {
    pub fn is_in_wait_list(
        &mut self,
        laptop_id: Uuid,
    ) -> crate::Result<Option<TransfertPendingEntry>> {
        use crate::views::v_transfert_en_cours::dsl::*;
        Ok(Some(
            v_transfert_en_cours
                .filter(laptop.eq(&laptop_id))
                .filter(point_vente.eq(&self.point_vente))
                .select(TransfertPendingEntry::as_select())
                .load(self.conn)?
                .first()
                .cloned()
                .ok_or(Error::NotInWaitingList(laptop_id))?,
        ))
    }
    fn validate<D: Into<PrimitiveDateTime>>(
        &mut self,
        laptop_id: Uuid,
        date: Option<D>,
    ) -> crate::Result<EntreeStockPointVente> {
        if let Some(entry) = self.is_in_wait_list(laptop_id)? {
            use crate::schema::entree_stock_point_vente::dsl::*;
            let to_insert = EntreeStockPointVente {
                id_entree_stock: Uuid::new_v4(),
                sortie_stock: entry.id_demande,
                entree_date: date.map(|d| d.into()),
            };
            Ok(diesel::insert_into(entree_stock_point_vente)
                .values(&to_insert)
                .returning(EntreeStockPointVente::as_returning())
                .get_result(self.conn)?)
        } else {
            Err(crate::Error::ValidateTransfertModule(
                Error::NotInWaitingList(laptop_id),
            ))
        }
    }
    pub fn validate_transfert<D: Into<PrimitiveDateTime>>(
        &mut self,
        laptop_id: Uuid,
        date: Option<D>,
    ) -> crate::Result<PointVenteStockEntry> {
        let entry = self.validate(laptop_id, date)?;
        use crate::views::v_point_vente_stock::dsl::*;
        Ok(v_point_vente_stock
            .filter(id_entree_stock.eq(entry.id_entree_stock))
            .select(PointVenteStockEntry::as_select())
            .get_result(self.conn)?)
    }
}

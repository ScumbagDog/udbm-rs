#include "dbm/dbm.h"

extern "C" {

  raw_t rs_dbm_boundbool2raw(int32_t bound, bool is_strict) {
    return dbm_boundbool2raw(bound, is_strict);
  }

  int32_t rs_dbm_raw2bound(raw_t raw) {
    return dbm_raw2bound(raw);
  }

  bool rs_dbm_rawIsStrict(raw_t raw) {
    return dbm_rawIsStrict(raw);
  }

  void rs_dbm_init(raw_t *dbm, cindex_t dim) {
    dbm_init(dbm, dim);
  }

  void rs_dbm_zero(raw_t *dbm, cindex_t dim) {
    dbm_zero(dbm, dim);
  }

  unsigned int rs_dbm_relation(const raw_t *dbm1, const raw_t *dbm2, cindex_t dim) {
    return dbm_relation(dbm1, dbm2, dim);
  }

  unsigned int rs_dbm_base_SUBSET() {
    return base_SUBSET;
  }

  bool rs_dbm_satisfies(const raw_t *dbm, cindex_t dim, cindex_t i, cindex_t j, raw_t constraint) {
    return dbm_satisfies(dbm, dim, i, j, constraint);
  }

  bool rs_dbm_close(raw_t *dbm, cindex_t dim) {
    return dbm_close(dbm, dim);
  }

  void rs_dbm_up(raw_t *dbm, cindex_t dim) {
    return dbm_up(dbm, dim);
  }

  void rs_dbm_down(raw_t *dbm, cindex_t dim) {
    return dbm_down(dbm, dim);
  }
  bool rs_dbm_constrain1(raw_t *dbm, cindex_t dim, cindex_t i, cindex_t j, raw_t constraint) {
    return dbm_constrain1(dbm, dim, i, j, constraint);
  }

  void rs_dbm_freeClock(raw_t *dbm, cindex_t dim, cindex_t k) {
    dbm_freeClock(dbm, dim, k);
  }

  void rs_dbm_updateValue(raw_t *dbm, cindex_t dim, cindex_t x, int32_t value) {
    dbm_updateValue(dbm, dim, x, value);
  }

  void rs_dbm_updateClock(raw_t *dbm, cindex_t dim, cindex_t x, cindex_t y) {
    dbm_updateClock(dbm, dim, x, y);
  }

  void rs_dbm_updateIncrement(raw_t *dbm, cindex_t dim, cindex_t x, int32_t value) {
    dbm_updateIncrement(dbm, dim, x, value);
  }


}

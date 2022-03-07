#ifndef WRAPPER_H
#define WRAPPER_H
#include "dbm/dbm.h"

extern "C" {
  void rs_dbm_init(raw_t *dbm, cindex_t dim);

  unsigned int rs_dbm_relation(const raw_t *dbm1, const raw_t *dbm2, cindex_t dim);
  bool rs_dbm_satisfies(const raw_t *dbm, cindex_t dim, cindex_t i, cindex_t j, raw_t constraint);

  bool rs_dbm_close(raw_t *dbm, cindex_t dim);

  void rs_dbm_up(raw_t *dbm, cindex_t dim);
  void rs_dbm_down(raw_t *dbm, cindex_t dim);
}

#endif //WRAPPER_H

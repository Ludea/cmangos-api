#include "cmangos-api/mangos-classic/src/mangosd/Master.h"

extern "C++" {
  int start_cmangos_server() {
    return Master::Instance().Run();
  }
}

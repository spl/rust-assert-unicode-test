#pragma once
#include <cassert>

class val {
    int mv;
public:
    val(int v) : mv(v) { }
    void do_assert() { assert(mv > 0); }
};

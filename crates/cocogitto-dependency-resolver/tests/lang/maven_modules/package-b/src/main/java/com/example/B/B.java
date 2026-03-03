package com.example.B;

import com.example.C.C;

public class B {
    public String hello() {
        C c = new C();
        return c.hello() + "and from B";
    }
}
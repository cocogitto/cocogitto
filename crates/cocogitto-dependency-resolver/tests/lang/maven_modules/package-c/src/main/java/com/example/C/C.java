package com.example.C;

import com.example.D.D;
import com.example.E.E;

public class C {
    public String hello() {

        D d = new D();
        E e = new E();
        return d.hello() + "and" + e.hello() + "and from C";
    }
}
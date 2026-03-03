package com.example.D;

import com.example.E.E;

public class D {
    public String hello() {
        E e = new E();
        String hello = e.hello();
        return hello + " and D";
    }
}
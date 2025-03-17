package com.ricardo.others;

import org.jeasy.rules.api.*;
import org.jeasy.rules.core.*;

public class RulesEngineE {
    public static void main(String[] args) {
        Rules rules = new Rules();
        rules.register(new SpendingRule(150)); // Example: Customer spent $150

        RulesEngine rulesEngine = new DefaultRulesEngine();
        rulesEngine.fire(rules, new Facts());
    }
}

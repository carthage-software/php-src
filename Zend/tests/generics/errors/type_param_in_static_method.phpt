--TEST--
Errors: class type parameter cannot be used in a static method signature
--FILE--
<?php
class A<T> {
    public static function foo(T $a): void {}
}
?>
--EXPECTF--
Fatal error: Non-static type parameter T cannot be referenced from a static context in %s on line %d

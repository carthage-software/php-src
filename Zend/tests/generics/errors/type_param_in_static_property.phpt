--TEST--
Errors: class type parameter cannot be used in a static property type
--FILE--
<?php
class A<T> {
    public static null|T $a = null;
}
?>
--EXPECTF--
Fatal error: Non-static type parameter T cannot be referenced from a static context in %s on line %d

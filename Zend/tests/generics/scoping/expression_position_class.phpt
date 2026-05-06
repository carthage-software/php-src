--TEST--
Scoping: T in expression position refers to class T (not type parameter)
--FILE--
<?php
class T {
    public int $val = 42;
}
function f<T : object>($x): int {
    $t = new T;
    return $t->val;
}
echo f(null), "\n";
?>
--EXPECT--
42

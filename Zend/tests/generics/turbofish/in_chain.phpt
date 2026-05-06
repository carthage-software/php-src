--TEST--
Turbofish: chained calls
--FILE--
<?php
class C {
    public function m() { return $this; }
    public function n($x) { return $x; }
}
$c = new C;
var_dump($c->m::<int>()->n::<string>("hello"));
?>
--EXPECT--
string(5) "hello"

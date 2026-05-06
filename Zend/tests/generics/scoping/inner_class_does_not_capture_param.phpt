--TEST--
Scoping: top-level class declaration inside body does not capture (already declared at file scope)
--FILE--
<?php
class T {
    public int $val = 99;
}

class Outer<T : object> {
    public T $tval;
    public function get(): T { return $this->tval; }
}

$ro = new ReflectionClass('Outer');
echo $ro->getProperty('tval')->getType()->getName(), "\n";

$x = new T;
echo $x->val, "\n";
?>
--EXPECT--
object
99

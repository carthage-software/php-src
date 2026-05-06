--TEST--
Turbofish: new with type arguments
--FILE--
<?php
class Box {
    public int $x;
    public function __construct(int $x) { $this->x = $x; }
}
$b = new Box::<int>(42);
echo $b->x, "\n";
?>
--EXPECT--
42

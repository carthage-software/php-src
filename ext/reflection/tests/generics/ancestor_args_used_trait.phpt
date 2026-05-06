--TEST--
Reflection: getGenericArgumentsForUsedTrait returns args from use clause
--FILE--
<?php
trait Holder<T> { public T $val; }
trait Pair<K, V> { public K $k; public V $v; }
trait Plain { public int $x; }

class WithArgs { use Holder<string>; }
class NoArgs   { use Holder; }
class NoTrait  {}
class Multi    { use Holder<bool>, Pair<int, float>; }
class Combo    { use Holder<string>, Plain; }

$rc = new ReflectionClass('WithArgs');
$args = $rc->getGenericArgumentsForUsedTrait('Holder');
echo "WithArgs/Holder: ", $args === null ? "null" : implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";

$rc = new ReflectionClass('NoArgs');
$args = $rc->getGenericArgumentsForUsedTrait('Holder');
echo "NoArgs/Holder: ", $args === null ? "null" : "non-null", "\n";

$rc = new ReflectionClass('NoTrait');
$args = $rc->getGenericArgumentsForUsedTrait('Holder');
echo "NoTrait/Holder: ", $args === null ? "null" : "non-null", "\n";

$rc = new ReflectionClass('Multi');
echo "Multi/Holder: ", implode(",", array_map(fn($t)=>$t->getName(), $rc->getGenericArgumentsForUsedTrait('Holder'))), "\n";
echo "Multi/Pair: ", implode(",", array_map(fn($t)=>$t->getName(), $rc->getGenericArgumentsForUsedTrait('Pair'))), "\n";

$rc = new ReflectionClass('Combo');
echo "Combo/Holder: ", implode(",", array_map(fn($t)=>$t->getName(), $rc->getGenericArgumentsForUsedTrait('Holder'))), "\n";
echo "Combo/Plain: ", $rc->getGenericArgumentsForUsedTrait('Plain') === null ? "null" : "non-null", "\n";

// Case insensitive
echo "case-insensitive: ", $rc->getGenericArgumentsForUsedTrait('holder')[0]->getName(), "\n";
?>
--EXPECT--
WithArgs/Holder: string
NoArgs/Holder: null
NoTrait/Holder: null
Multi/Holder: bool
Multi/Pair: int,float
Combo/Holder: string
Combo/Plain: null
case-insensitive: string

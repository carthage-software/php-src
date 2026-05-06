--TEST--
Reflection: getGenericArgumentsForParentInterface returns args from implements / interface-extends
--FILE--
<?php
interface I<K, V> {}
interface J<T> {}
interface K1 extends I<int, string> {}

class WithArgs implements I<int, string> {}
class WithoutArgs implements I {}
class NotImplements {}
class Multi implements I<bool, float>, J<string> {}

// Lookup hits
$rc = new ReflectionClass('WithArgs');
echo "WithArgs/I: ";
$args = $rc->getGenericArgumentsForParentInterface('I');
echo $args === null ? "null" : implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";

// Same name, different case
echo "WithArgs/i (case insensitive): ";
$args = $rc->getGenericArgumentsForParentInterface('i');
echo $args === null ? "null" : implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";

// Implements without args
echo "WithoutArgs/I: ";
var_dump($rc = new ReflectionClass('WithoutArgs'));
$args = $rc->getGenericArgumentsForParentInterface('I');
echo $args === null ? "null" : "non-null", "\n";

// Not in implements list
echo "NotImplements/I: ";
$args = (new ReflectionClass('NotImplements'))->getGenericArgumentsForParentInterface('I');
echo $args === null ? "null" : "non-null", "\n";

// Multiple interfaces
$rc = new ReflectionClass('Multi');
echo "Multi/I: ";
$args = $rc->getGenericArgumentsForParentInterface('I');
echo $args === null ? "null" : implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";

echo "Multi/J: ";
$args = $rc->getGenericArgumentsForParentInterface('J');
echo $args === null ? "null" : implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";

// Interface extending interface (also queryable through this method)
echo "K1/I: ";
$args = (new ReflectionClass('K1'))->getGenericArgumentsForParentInterface('I');
echo $args === null ? "null" : implode(",", array_map(fn($t)=>$t->getName(), $args)), "\n";
?>
--EXPECTF--
WithArgs/I: int,string
WithArgs/i (case insensitive): int,string
WithoutArgs/I: object(ReflectionClass)#%d (1) {%a}
null
NotImplements/I: null
Multi/I: bool,float
Multi/J: string
K1/I: int,string

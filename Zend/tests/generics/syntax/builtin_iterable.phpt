--TEST--
Generic syntax: iterable<...>
--FILE--
<?php
function f(iterable<string> $x): iterable<int, string> { return []; }
$r = new ReflectionFunction('f');
$pt = $r->getParameters()[0]->getType();
echo $pt->getName(), " args=", count($pt->getGenericArguments()), "\n";
$rt = $r->getReturnType();
echo $rt->getName(), " args=", count($rt->getGenericArguments()), "\n";
?>
--EXPECT--
iterable args=1
iterable args=2

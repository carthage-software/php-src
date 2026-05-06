--TEST--
Reflection: getGenericArgumentsForParentClass returns args from extends clause
--FILE--
<?php
class A<T> {}
class B<X, Y> {}

class WithArgs extends A<string> {}
class NoArgs extends A {}
class NoParent {}
class Multi extends B<int, float> {}

$cases = [
    'WithArgs' => ['string'],
    'NoArgs'   => null,
    'NoParent' => null,
    'Multi'    => ['int', 'float'],
];

foreach ($cases as $cls => $want) {
    $args = (new ReflectionClass($cls))->getGenericArgumentsForParentClass();
    if ($want === null) {
        echo $cls, ": ", $args === null ? "null OK" : "FAIL (got non-null)", "\n";
    } else {
        $got = array_map(fn($t) => $t->getName(), $args);
        echo $cls, ": ", $got === $want ? "OK" : ("FAIL got " . implode(",", $got)), "\n";
    }
}
?>
--EXPECT--
WithArgs: OK
NoArgs: null OK
NoParent: null OK
Multi: OK

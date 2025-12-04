--TEST--
ReflectionTypeAlias class
--FILE--
<?php

namespace App\Types;

type UserId = int|string;

$ref = new \ReflectionTypeAlias('App\Types\UserId');

var_dump($ref->getName());
var_dump($ref->getShortName());
var_dump($ref->getNamespaceName());
var_dump($ref->inNamespace());

$type = $ref->getType();
var_dump($type instanceof \ReflectionUnionType);
var_dump((string)$type);

?>
--EXPECT--
string(16) "App\Types\UserId"
string(6) "UserId"
string(9) "App\Types"
bool(true)
bool(true)
string(10) "string|int"

--TEST--
Reflection: ReflectionGenericVariance enum cases
--FILE--
<?php
echo ReflectionGenericVariance::Invariant->value, "\n";
echo ReflectionGenericVariance::Covariant->value, "\n";
echo ReflectionGenericVariance::Contravariant->value, "\n";
?>
--EXPECT--
0
1
2

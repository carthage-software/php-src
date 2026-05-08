--TEST--
Diamond: class implementing two interfaces that bind the same generic ancestor with different args is rejected
--FILE--
<?php
interface Box<T> {}
interface ABox extends Box<int> {}
interface BBox extends Box<string> {}

class C implements ABox, BBox {}
?>
--EXPECTF--
Fatal error: C inherits Box<int> via ABox and Box<string> via BBox in %s on line %d

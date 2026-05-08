--TEST--
Diamond: parent and direct interface providing different bindings for same generic ancestor is rejected
--FILE--
<?php
interface Box<T> {}
class P implements Box<int> {}

class C extends P implements Box<string> {}
?>
--EXPECTF--
Fatal error: C inherits Box<int> via P and Box<string> via Box in %s on line %d

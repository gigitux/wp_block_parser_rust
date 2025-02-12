<?php

require_once __DIR__ . '/parser.php';

$document = "<!-- wp:paragraph --><p>Hello world</p><!-- /wp:paragraph -->";
$parser = new WP_Block_Parser();
$blocks = $parser->parse($document);

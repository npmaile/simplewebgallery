<html>
	<head>
	<script src="script.js"></script>
<script>
var imglist=[
<?php
if (!empty($_GET)){
	$newpath = str_replace('"','',$_GET['path']);
	chdir($newpath);
}
else{
	chdir($_SERVER['DOCUMENT_ROOT']);
}
$docroot = $_SERVER['DOCUMENT_ROOT'];

$baddata = shell_exec("bash $docroot/advpicsite.sh");
echo(str_replace('//','/',str_replace('/secure/','/photos/',str_replace('//','/',str_replace($docroot,'/',$baddata)))));
?>
]
	</script>
	</head>
	<body>
		<p id="gallery">
					
		<p>
	</body>
</html>


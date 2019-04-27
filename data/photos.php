<html>
	<head>
	<link rel="stylesheet" href="style.css">
	<script src="script.js"/></script>
<script>
var medialist=[
<?php

$mediaPatterns;

if (!empty($_GET['media'])){
	$mediaPatterns = $_GET['media'];
}


$rootdir = $_SERVER['DOCUMENT_ROOT'];
$workingdir = $rootdir;
if (!empty($_GET['path'])){
	$requestpath = urldecode($_GET['path']);
	$workingdir = ("$rootdir/$requestpath");
}





function fileextension($name){
return substr($name, strrpos($name, '.')+1, strlen($name)-strrpos('.', $name));
}


function getfiles($rootpath,$patterns){
	$ret = array();
	foreach (scandir($rootpath) as $file){
		$pathToCheck = "$rootpath/$file";	
		if(substr($pathToCheck, -2) == "/."){
		}
		elseif(substr($pathToCheck, -3) == "/.."){
		}
		elseif(is_dir("$pathToCheck")){
			$ret = array_merge($ret,getfiles("$pathToCheck",$patterns));
		}
		elseif(in_array(fileextension("$pathToCheck"),$patterns)){
			array_push($ret,"$pathToCheck");
		}
	}
	return $ret;
}

$filesToDisplay = getfiles($workingdir, $mediaPatterns);
shuffle($filesToDisplay);	
foreach ($filesToDisplay as $file){
	$mediaLink = str_replace("$rootdir/",'',$file);	
	echo("\"$mediaLink\",\n");
}



?>
]
	</script>
	</head>
	<body>
		<p id="gallery">
					
		<p>
	</body>
</html>


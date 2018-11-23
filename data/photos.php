<html>
	<head>
	<script src="script.js"></script>
<script>
var imglist=[
<?php
$rootdir = $_SERVER['DOCUMENT_ROOT'];
$workingdir = $rootdir;
if (!empty($_GET)){
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
$photoPatterns = array("jpg","png", "jpeg", "gif");
	$filesToDisplay = getfiles($workingdir, $photoPatterns);
	shuffle($filesToDisplay);	
	foreach ($filesToDisplay as $image){
	$imgLink = str_replace("$rootdir/",'',$image);	
	echo("\"$imgLink\",\n");
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


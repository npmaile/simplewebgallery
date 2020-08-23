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
		<?php

			function PhotoLink($link){
				echo("<a href=/index.php?path=$link&media[]=jpg&media[]=jpeg&media[]=png&media[]=gif> photos</a>");
			}

			function IndexLink($link,$path){
				
				echo("<a href=/index.php?path=$link> $path </a>");
			}

			function VidLink($link){
				echo("<a href=/index.php?path=$link&media[]=mp4&media[]=wmv&media[]=webm> videos</a>");
			}


			$webroot = $_SERVER['DOCUMENT_ROOT'];
			$workingpath = $webroot;
			if (!empty($_GET)){
				$requestpath = urldecode($_GET['path']);
				$workingpath = "$webroot/$requestpath";
			}
			$fakedirectories = scandir("$workingpath");
			$directories=array();
			foreach($fakedirectories as $file){
				if(is_dir("$workingpath/$file")){
				array_push($directories,$file);
				}
			}
			echo("<table><tr><th>path</th><th>photogallery</th><th>videogallery</th></tr>");
			foreach($directories as $directory){
			$cleandirref = str_replace("$webroot/",'',"$workingpath/$directory");
			$urlencodeddir = urlencode($cleandirref);	
			echo("<tr><th>");
			IndexLink($urlencodeddir, $cleandirref);
			echo("</th><th>");
			PhotoLink($urlencodeddir);
			echo("</th><th>");
			VidLink($urlencodeddir);
			echo("</th></tr>\n");
			}
			echo("</table>");
		?>
		<p id="gallery">
					
		<p>

	</body>
</html>

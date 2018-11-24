<html>
	<head>
		
	</head>	
	<body>
		<?php

			function PhotoLink($link){
				echo("<a href=/photos.php?path=$link&media[]=jpg&media[]=jpeg&media[]=png&media[]=gif> photos</a>");
			}

			function IndexLink($link,$path){
				
				echo("<a href=/index.php?path=$link> $path </a>");
			}

			function VidLink($link){
				echo("<a href=/photos.php?path=$link&media[]=mp4&media[]=wmv&media[]=webm> videos</a>");
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
			foreach($directories as $directory){
			$cleandirref = str_replace("$webroot/",'',"$workingpath/$directory");
			$urlencodeddir = urlencode($cleandirref);		
			echo("<br>");
			IndexLink($urlencodeddir, $cleandirref);
			PhotoLink($urlencodeddir);
			VidLink($urlencodeddir);
			echo("</br>\n");
			}
		?>
	</body>
</html>

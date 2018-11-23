<html>
	<head>
		
	</head>	
	<body>
		<?php
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
			echo("<br><a href=/index.php?path=$urlencodeddir>$directory</a><a href=/photos.php?path=$urlencodeddir> photos</a></br>");
			}

		?>
	</body>
</html>

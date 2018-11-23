<html>
	<head>
		
	</head>	
	<body>
<?php
			if (!empty($_GET)){
				
				$newpath = urldecode(str_replace('"','',$_GET['path']));
				chdir($newpath);
			}
			else{
				chdir($_SERVER['DOCUMENT_ROOT']);
			}
			$WorkDir = getcwd();
			$fakedirectories = scandir($WorkDir);
			$directories=array();
			foreach($fakedirectories as $file){
				if(is_dir($file)){
				array_push($directories,$file);
				}
			}
			foreach($directories as $directory){
			$cleandirref = "$WorkDir/$directory";
			$urlencodeddir = urlencode($cleandirref);
			echo("<br><a href=/index.php?path=$urlencodeddir>$WorkDir/$directory</a><a href=/photos.php?path=$urlencodeddir> photos</a></br>");
			}

		?>
	</body>
</html>

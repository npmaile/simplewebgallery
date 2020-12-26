var iter = 0
var medialist = []

var medialimit

setMediaHeight = function(){
	var height = window.innerHeight;
	var medias = document.getElementsByClassName('image video-js');
	for (var i = 0; i < medias.length; i++) {
		medias[i].style.height = height + 'px';
	}
}

addimage = function(image,id){
	var paragraph = document.getElementById("gallery");
	var newimage = document.createElement("img");
	newimage.setAttribute('src',encodeURIComponent(image));
	newimage.setAttribute('id', id);
	newimage.setAttribute('class','image');
	paragraph.appendChild(newimage);
}

addvideo = function(video,id){
	var paragraph = document.getElementById("gallery");
	var newvid = document.createElement("video");
	var source = document.createElement("source");
	source.setAttribute('src',encodeURIComponent(video));
	newvid.appendChild(source);
	newvid.setAttribute('id',id);
	newvid.setAttribute('class','video-js');
	newvid.setAttribute('data-setup','{"controls":"true"}');
	newvid.setAttribute('fluid','true');
	paragraph.appendChild(newvid);
}

genMedia = function(num){
	while(num > 0 && iter < medialist.length){
		if (/.*\.(jpeg|jpg|gif|png)$/.test(medialist[iter])){
			addimage(medialist[iter]);
		}
		else if (/.*\.(mp4|mov|webm)$/.test(medialist[iter])){
			addvideo(medialist[iter],iter);
		}
		iter ++;
		num --;
	}
}

loadmoaronbottom = function(){
	var buffer = 20
	var progress, totalheight;
	totalheight = document.body.scrollHeight;
	progress = window.scrollY + window.innerHeight;
	if(totalheight <= progress + buffer){
		genMedia(10);
	}
}

window.onresize = setMediaHeight;
window.onscroll = loadmoaronbottom;
var alreadyloadedonce = 0
window.onload = function(){
	if (alreadyloadedonce == 0){
		genMedia(20);
		alreadyloadedonce = 1;
	}

}

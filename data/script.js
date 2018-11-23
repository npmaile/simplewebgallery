var iter = 0
var imglist = []

var photolimit

setPhotoHeight = function(){
	var height = window.innerHeight;
	var vids = document.getElementsByClassName('vidsbeeyotch');
	for (var i = 0; i < vids.length; i++) {
		vids[i].style.height = height + 'px';
	}
}

createnewimage = function(num){
	var paragraph = document.getElementById("gallery")
	while(num > 0 && iter < imglist.length){
		var newimage = document.createElement("img");
		newimage.setAttribute('src',imglist[iter]);
		newimage.setAttribute('id', iter);
		newimage.setAttribute('class','vidsbeeyotch');
		newimage.setAttribute('height',window.innerHeight)
		paragraph.appendChild(newimage);
		iter ++;
		num --;
	}
}

loadmoaronbottom = function(){
	var buffer = 10
	var progress, totalheight;
	totalheight = document.body.scrollHeight;
	progress = window.scrollY + window.innerHeight;
	if(totalheight <= progress + buffer){
		createnewimage(10)
	}
}

deleteimage = function(){
	var image = document.getElementById(iter-photolimit);
}
window.onresize = setPhotoHeight;
window.onscroll = loadmoaronbottom;
var alreadyloadedonce = 0
window.onload = function(){
	if (alreadyloadedonce == 0){
		createnewimage(10);
		alreadyloadedonce = 1;
	}

}

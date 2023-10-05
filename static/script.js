var iter = 0
var medialist = []
var medialist

setMediaHeight = function() {
	var height = window.innerHeight;
	var medias = document.getElementsByClassName('image video-js');
	for (var i = 0; i < medias.length; i++) {
		medias[i].style.height = height + 'px';
	}
}

addimage = function(image) {
	var idString = "v" + Math.random().toString(36).slice(2,35)
	var paragraph = document.getElementById("gallery");
	var newimage = document.createElement("img");
	newimage.setAttribute('src', "data" + image);
	newimage.setAttribute('id', idString);
	newimage.setAttribute('class', 'image');
	paragraph.appendChild(newimage);
}

addvideo = function(video) {
	var idString = "v" + Math.random().toString(36).slice(2,35)
	var paragraph = document.getElementById("gallery");
	var newvid = document.createElement("video");
	var source = document.createElement("source");
	source.setAttribute('src', "data" + video);
	newvid.appendChild(source);
	newvid.setAttribute('id', idString);
	newvid.setAttribute('class', 'video-js vjs-default-skin');
	newvid.setAttribute('loop', "true");

	var container = document.createElement("div");
	container.appendChild(newvid)
	container.setAttribute('class','video-container')
	paragraph.appendChild(container);

	var title = document.createElement("div");
	title.innerHTML = video;
	title.setAttribute('class', 'title');
	paragraph.appendChild(title);
	videojs(idString,{
		"controls":true, "preload": false, "fill":true
	});
}

genMedia = function(num) {
	while (num > 0 && iter < medialist.length) {
		if (/.*\.(jpeg|jpg|gif|png)$/.test(medialist[iter].toLowerCase())) {
			addimage(medialist[iter]);
		}
		else if (/.*\.(m4v|mp4|mov|webm)$/.test(medialist[iter].toLowerCase())) {
			addvideo(medialist[iter]);
		}
		iter++;
		num--;
	}
}

loadmoaronbottom = function() {
	var buffer = 20
	var progress, totalheight;
	totalheight = document.body.scrollHeight;
	progress = window.scrollY + window.innerHeight;
	if (totalheight <= progress + buffer) {
		genMedia(buffer);
	}
}

function spaLink(element, apilink) {
	element.addEventListener('click', function(event) {
		event.preventDefault();
		reloadsite(apilink)
	})
}

function addDirsToList(listing, dirs) {
	list = document.createElement("ul")
	listing.appendChild(list)
	for (let i = 1; i < dirs.length; i++) {
		li = document.createElement("li")
		a = document.createElement("a")
		a.innerHTML = dirs[i]
		a.href = "api" + dirs[i]
		a.addEventListener('click', function(event) {
			event.preventDefault();
			reloadsite("api" + dirs[i] + "?dir_depth=1")
		})
		li.appendChild(a)

		a = document.createElement("button")
		a.innerHTML = "     photos"
		a.href = "api" + dirs[i]
		a.addEventListener('click', function(event) {
			event.preventDefault();
			reloadsite("api" + dirs[i] + "?media_extensions=jpg,png,jpeg,gif&dir_depth=1")
		})
		li.appendChild(a)

		a = document.createElement("button")
		a.innerHTML = "     videos"
		a.href = "api" + dirs[i]
		a.addEventListener('click', function(event) {
			event.preventDefault();
			reloadsite("api" + dirs[i] + "?media_extensions=m4v,mp4,mov,webm&dir_depth=1")
		})
		li.appendChild(a)

		list.appendChild(li)
	}
}

async function generateSite(url) {
	res = await fetch(url);
	txt = await res.text()
	data = JSON.parse(txt)
	head = document.getElementById("current_dir")
	splitParts = data.current_dir.split("/")
	for (let i = 0; i < splitParts.length; i++) {
		a = document.createElement("a")
		if (i == 0) {
			a.innerHTML = "root"
			a.href = "api/"
			spaLink(a, "api/?dir_depth=1")
			head.appendChild(a)
			continue
		}
		a.innerHTML = "/" + splitParts[i]
		a.href = "api" + splitParts.slice(0, i + 1).join("/")
		a.addEventListener('click', function(event) {
			event.preventDefault();
			reloadsite("api" + splitParts.slice(0, i + 1).join("/") + "?dir_depth=1")
		})
		head.appendChild(a)
	}

	plink = document.createElement("button")
	plink.innerHTML = "    photos"
	spaLink(plink, "api" + data.current_dir + "?media_extensions=jpg,png,jpeg,gif&dir_depth=1")
	head.appendChild(plink)

	vlink = document.createElement("button")
	vlink.innerHTML = "    videos"
	spaLink(vlink, "api" + data.current_dir + "?media_extensions=m4v,mp4,mov,webm&dir_depth=1")
	head.appendChild(vlink)

	dirsListing = document.getElementById("dirs")
	addDirsToList(dirsListing, data.dirs)
	medialist = data.files
	iter = 0;
	genMedia(20);
}

async function reloadsite(url) {
	/*
	//murder all videojs videos
	if (videojs.players.length != undefined){
		let players = videojs.players
		let playerNames = Object.keys(players)
		for (let i = 0; i < playerNames.length; i++){
			player = players[playerNames[i]]
			if player != undefined{
				player.dispose()
			}
		}
	}
	*/
	
	toclear = ["current_dir", "dirs", "gallery"]
	for (let i = 0; i < toclear.length; i++) {
		x = document.getElementById(toclear[i])
		for (let j = 0; j < x.children; x++) {
			c.removeChild(x.children[0])
		}
		x.innerHTML = ""
	}
	medialist = []
	alreadyloadedonce = 0
	if (document.getElementById('randomizeCheckbox').checked) {
		beginning = new URL(window.location.origin)
		const urlWithParams = new URL(url, beginning);
		urlWithParams.searchParams.append("file_sort", "random");
		url = urlWithParams.href
	}
	await generateSite(url)
	if (alreadyloadedonce == 0) {
		genMedia(20);
		alreadyloadedonce = 1;
	}
}

let keepautoscrolling = false

function autoscroll() {
	window.scrollBy(0, 1);
	if (!keepautoscrolling) {
		return
	}
	scrolldelay = setTimeout(autoscroll, 10);
}

function setupAutoScrolling() {
	let sb = document.getElementById("scrollbutton")
	sb.addEventListener('click', function(_event) {
		keepautoscrolling = true
		autoscroll()
	})
	let hw = document.getElementById("stopscrollbutton")
	hw.addEventListener('click', function(_event) {
		keepautoscrolling = false
	})
	let top = document.getElementById("returnToTop")
	top.addEventListener('click', function(_event) {
		window.scrollTo(0, 0)
	})
}

window.onresize = setMediaHeight;
window.onscroll = loadmoaronbottom;
var alreadyloadedonce = 0

window.onload = function() {
	reloadsite("api/?dir_depth=1")
	setupAutoScrolling()
}




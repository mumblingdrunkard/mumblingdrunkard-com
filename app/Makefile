default:
	npm run build --omit=dev
	sed 's/\.\//\/app\/static\/build\//' public/app/static/build/main.js > tmp.js
	sh inject.sh > public/index.html
	rm public/app/static/build/main.js tmp.js

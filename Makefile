git:
	git add -A
	git commit -m "Update"
	git push
publish:
	cd mkpm_client && cargo release --exec
.PHONY: contract-deploy
ADMIN_KEY := $(shell stellar keys address admin)

stellar-admin-secret-get:
	stellar keys secret admin

stellar-admin-create:
	stellar keys generate admin --network testnet --fund

contract-deploy:
	stellar contract deploy \
	--wasm target/wasm32v1-none/release/seed_incubataor.wasm \
	--source admin \
	--network testnet \
	--alias seed_incubataor \
	-- \
	--admin $(ADMIN_KEY)
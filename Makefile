store:
  archway contracts store nameservice

init:
  archway contracts instantiate nameservice --args '{"purchase_price":{"amount":"100","denom":"aconst"}}'

register:
  archway contracts execute nameservice --args '{"register":{"name":"fred"}}' --amount 100aconst

query:
	archway contracts query smart nameservice --args '{"resolve_record": {"name": "fred"}}'

transfer:
	archway contracts execute nameservice --args '{"transfer":{"name":"fred","to":"archway1htum43he4n46gdmvuzr72ahsyvau4ummdgeytv"}}' --amount 100aconst

.PHONY: store

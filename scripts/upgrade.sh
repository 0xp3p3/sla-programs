anchor build
anchor upgrade ./target/deploy/sla.so --program-id GUSxqUfUdqchfErA3DrW1jNVJKGdMpxt71AeDkJJtG5R

cp ./target/idl/sla.json ../sla-frontend/sla_idl.json
cp ./target/idl/sla.json ../sla-config/programs/sla_idl.json


# Build and push to cachix
build-cachix auth_token:
    CACHIX_AUTH_TOKEN="{{auth_token}}" \
    nix build .#default --no-link --print-out-paths \
      | cachix push ultraflame4

RELEASE_DIR="target/release"

cargo build --release
mkdir -p bin
for b in wt wt_n_bench bv_bench rank_bench sel_bench wt_access_bench wt_select_bench wt_sigma_bench
do
    cp $RELEASE_DIR/$b bin/.
done

zip -r bin.zip bin

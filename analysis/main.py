import argparse
import pandas as pd
import process
import visualization
import statistical_visualization

def main():
    parser = argparse.ArgumentParser(description='Run different types of visualizations.')
    parser.add_argument('--type', type=str, choices=['statistical', 'single', 'analysis'], required=True,
                        help='Type of visualization to run. Choose "statistical" for mean and std, "single" for individual runs.')

    args = parser.parse_args()

    if args.type == 'statistical':
        simulation_data = process.import_all_csvs('../output')
        [means, stds] = process.compute_mean_and_std(simulation_data)
        statistical_visualization.plot_all(means.astype(float), stds.astype(float), 'combined_plot_statistical.png')
    elif args.type == 'single':
        simulation_data = process.import_wad_csv('../output/portfolio_0.csv')
        visualization.plot_all(simulation_data, 'combined_plot_single.png', start_index=0, end_index=100)
    elif args.type == 'analysis':
        data = process.read_in_sweep('../output')
        # data = pd.read_csv('sweep_results.csv')
        statistical_visualization.plot_heatmaps(data)

if __name__ == "__main__":
    main()
